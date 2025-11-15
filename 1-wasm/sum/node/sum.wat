(module
    (import "./calc-api.js" "logStart" (func $log_start))
    (import "./calc-api.js" "logOperation" (func $log_operation (param i32)))
    (import "./calc-api.js" "logResult" (func $log_result (param i32)))

    (func $sum (param $a i32) (param $b i32) (result i32)
        (local $result i32)

        call $log_start

        local.get $a
        call $log_operation

        local.get $b
        call $log_operation

        local.get $a
        local.get $b
        i32.add
        local.tee $result

        call $log_result

        local.get $result
    )

    (export "sum" (func $sum))
)
