type Test_Result (
    passed: Bool
)

type Describe_Result (
    passed: Bool
    // it_results: List<It_Result>
)

type It_Result (
    passed: Bool
)


fun test(message: String, body: fun()) {
    // enter scope
    body()
    // leave scope
}

fun describe(message: String, body: fun()) {
    let counter = 0
    // enter scope
    body()
    // leave scope
}

fun it(message: String, body: fun() -> Bool){
    if body() {
        std::io::print_line('Test passed')
    } else {
        std::io::print_line('Test failed')
    }
}


fun a(){
    std::io::print_line('I will pass')
    return true
}

fun b(){
    std::io::print_line('I will fail')
    return false
}


fun two(){
    it('passes', a)
    it('fails',  b)
}

fun one(){
    describe('desc', two)
}

test('test', one)

// std::io::print_line(result.passing_count)
// std::io::print_line(result.failing_count)