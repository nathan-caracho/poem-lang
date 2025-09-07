use std

fn sum  (a int) (b int) : int
  a+b

fn div a b : a/b

rail:
  sum 10 11
  div 20
on success value : print f"success {e}"
on error value : print f"error {e}"

