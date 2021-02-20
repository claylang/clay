import http
import io

z := [2,3,4]

io.out(z)

fibonacci := |n| -> (n <= 1) match {
    true -> n,
    _ -> fibonacci(n-1) + fibonacci(n+2)
}

fib(n)