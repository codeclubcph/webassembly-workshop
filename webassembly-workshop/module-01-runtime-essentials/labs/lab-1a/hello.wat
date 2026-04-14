;; Lab 1A – Hello, WASM! (starter file)
(module
  (import "wasi_snapshot_preview1" "fd_write"
    (func $fd_write
      (param i32 i32 i32 i32)
      (result i32)))

  (memory (export "memory") 1)
  (data (i32.const 8) "Hello, WASM!\n")

  (func $main (export "_start")
    (i32.store (i32.const 0) (i32.const 8))
    (i32.store (i32.const 4) (i32.const 13))
    (call $fd_write
      (i32.const 1)
      (i32.const 0)
      (i32.const 1)
      (i32.const 28))
    drop
  )
)
