;; Lab 1B – Linear Memory Explorer
(module
  (memory (export "memory") 1)

  (func (export "store") (param $addr i32) (param $val i32)
    (i32.store (local.get $addr) (local.get $val)))

  (func (export "load") (param $addr i32) (result i32)
    (i32.load (local.get $addr)))

  (func (export "mem_size") (result i32)
    memory.size)

  (func (export "mem_grow") (param $pages i32) (result i32)
    (memory.grow (local.get $pages)))
)
