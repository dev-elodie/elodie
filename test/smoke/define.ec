type Person (name: String)

define Person {
    function say_name(){
        std::io::println('Hi, I am ${self.name}')
    }
}

//let elodie = Elodie(name = 'Elodie')
//elodie.say_name()

// out: Hi, I am Elodie
