;; Lab 1C – Stack machine: compute (10 + 5) * 3 - 2
(module
  (func (export "compute") (result i32)
    i32.const 10
    i32.const 5
    i32.add
    i32.const 3
    i32.mul
    i32.const 2
    i32.sub
  )
)
