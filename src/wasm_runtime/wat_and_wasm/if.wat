(module
	(func $if_ie (export "if_ie") (result i32 i32)
		(i32.const 0)
		(if (result i32 i32)
			(then
				(i32.const 1)
				(i32.const 2))
			(else
				(i32.const 2)
				(i32.const 3)))))
