# Rayon

Rayon是一个第三方包，使用它可以将顺序计算转换为安全的并行计算，并且保证数据无竞争。Rayon提供了两种方法:

* **并行迭代器**：可以并行执行的迭代器。
* **join方法**：可以并行处理递归或分治风格的问题。

&nbsp;

## Rayon并行迭代器

```rust
use rayon::prelude::*;

fn sum_of_squares(input: &[i32]) -> i32 {
    // par_iter迭代器，Rayon提供的并行迭代器，它会返回一个不可变的并行迭代器类型
    input.par_iter().map(|&i| i * i).sum()
}

fn increment_all(input: &mut [i32]) {
    // par_iter_mut 迭代器，Rayon提供的可变并行迭代器
    input.par_iter_mut().for_each(|p| *p += 1);
}

fn main() {
    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let r = sum_of_squares(&v);
    println!("{}", r);

    let mut v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    increment_all(&mut v);
    println!("{:?}", v);
}
```

```x86asm
; sum_of_squares
Dump of assembler code for function eee::sum_of_squares:
   0x000055555556e820 <+0>:	sub    rsp,0x38
   0x000055555556e824 <+4>:	mov    QWORD PTR [rsp+0x28],rdi
   0x000055555556e829 <+9>:	mov    QWORD PTR [rsp+0x30],rsi
   0x000055555556e82e <+14>:	call   0x555555570f90 <<I as rayon::iter::IntoParallelRefIterator>::par_iter>
   0x000055555556e833 <+19>:	mov    QWORD PTR [rsp+0x18],rax
   0x000055555556e838 <+24>:	mov    QWORD PTR [rsp+0x20],rdx
   0x000055555556e83d <+29>:	mov    rsi,QWORD PTR [rsp+0x20]
   0x000055555556e842 <+34>:	mov    rdi,QWORD PTR [rsp+0x18]
   0x000055555556e847 <+39>:	call   0x55555556b820 <rayon::iter::ParallelIterator::map>
   0x000055555556e84c <+44>:	mov    QWORD PTR [rsp+0x8],rax
   0x000055555556e851 <+49>:	mov    QWORD PTR [rsp+0x10],rdx
   0x000055555556e856 <+54>:	mov    rsi,QWORD PTR [rsp+0x10]
   0x000055555556e85b <+59>:	mov    rdi,QWORD PTR [rsp+0x8]
   0x000055555556e860 <+64>:	call   0x555555573bf0 <rayon::iter::ParallelIterator::sum>
   0x000055555556e865 <+69>:	mov    DWORD PTR [rsp+0x4],eax
   0x000055555556e869 <+73>:	mov    eax,DWORD PTR [rsp+0x4]
   0x000055555556e86d <+77>:	add    rsp,0x38
=> 0x000055555556e871 <+81>:	ret
End of assembler dump.

; increment_all
Dump of assembler code for function eee::increment_all:
   0x000055555556e880 <+0>:	sub    rsp,0x28
   0x000055555556e884 <+4>:	mov    QWORD PTR [rsp+0x18],rdi
   0x000055555556e889 <+9>:	mov    QWORD PTR [rsp+0x20],rsi
=> 0x000055555556e88e <+14>:	call   0x555555570fc0 <<I as rayon::iter::IntoParallelRefMutIterator>::par_iter_mut>
   0x000055555556e893 <+19>:	mov    QWORD PTR [rsp+0x8],rax
   0x000055555556e898 <+24>:	mov    QWORD PTR [rsp+0x10],rdx
   0x000055555556e89d <+29>:	mov    rsi,QWORD PTR [rsp+0x10]
   0x000055555556e8a2 <+34>:	mov    rdi,QWORD PTR [rsp+0x8]
   0x000055555556e8a7 <+39>:	call   0x55555556b850 <rayon::iter::ParallelIterator::for_each>
   0x000055555556e8ac <+44>:	add    rsp,0x28
   0x000055555556e8b0 <+48>:	ret
End of assembler dump.
```

&nbsp;

## join并行迭代

`join` 方法并不一定会保证并行执行闭包， Rayon底层使用线程池来执行任务，如果工作线程被占用，Rayon会选择顺序执行。

Rayon的并行能力是基于一种**工作窃取**(Work-Stealing)技术，线程池中每个线程都有一个互不影响的任务队列(双向队列)，线程每次都从当前任务队列的头部取出一个任务来执行。如果某个线程对应的队列已空并且处于空闲状态，而其他线程的队列中还有任务需要处理，但是该线程处于工作状态，那么空闲的线程就可以从其他线程的队列尾部取一个任务来执行。这种行为表现就像空闲线程去偷工作中的线程任务一样，所以叫做**工作窃取**。

```rust
fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    let (a, b) = rayon::join (
        || fib(n-1), || fib(n-2)
    );
    a+b
}

fn main() {
    let r = fib(32);
    assert_eq!(r, 2178309);
}
```
