let result = loop {
    let x = 2
    if x > 1 {
        break x * 10
    }
}

std::io::print_line(result)

// out:20