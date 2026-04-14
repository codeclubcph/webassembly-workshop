;; Lab 1C – Solution: factorial(5) = 120
(module
  (func (export "factorial") (param $n i32) (result i32)
    (local $result i32)
    (local.set $result (i32.const 1))  ;; result = 1

    (block $break
      (loop $loop
        ;; if n <= 1, break
        (br_if $break (i32.le_s (local.get $n) (i32.const 1)))

        ;; result = result * n
        (local.set $result
          (i32.mul (local.get $result) (local.get $n)))

        ;; n = n - 1
        (local.set $n
          (i32.sub (local.get $n) (i32.const 1)))

        ;; continue loop
        (br $loop)
      )
    )

    local.get $result
  )
)
