use std::string::String;
use std::io::{BufReader, BufRead};
use std::fs::File;
//
//enum OpCode {
//    Add,
//    Multiply,
//    EndOfProgram,
//}
//
//struct Instruction {
//    opcode: Opcode,
//    operands: FixedSizeArray<i64, 3>,
//}

/// Parses the opcodes in a given string
fn parseInstructionsAndData(str: String) -> Result<(), String> {
    let mut split = str.split(",");
    let mut initial_data: Vec<i64> = Vec::new();
    for sub_str in split {
        match sub_str.parse::<i64>() {
            Err(e) => return Err(String::from("Int parsing failed!")),
            Ok(i) => initial_data.push(i),
        }
    }

    for noun in 0..100 {
        for verb in 0..100 {
            let mut data = initial_data.clone();

            data[1] = noun;
            data[2] = verb;

            let mut i = 0;
            while(i < data.len()){
//                println!("{}", i);
                match data[i] {
                    1 => {
                        let mut target_index = data[i+3] as usize;
                        let mut operand_index_1 = data[i+1] as usize;
                        let mut operand_index_2 = data[i+2] as usize;
                        data[target_index] = data[operand_index_1] + data[operand_index_2]
                    }
                    2 => {
                        let mut target_index = data[i+3] as usize;
                        let mut operand_index_1 = data[i+1] as usize;
                        let mut operand_index_2 = data[i+2] as usize;
                        data[target_index] = data[operand_index_1] * data[operand_index_2]
                    }
                    99 => break,
                    _ => return Err(String::from("Could not parse opcode!"))
                }
                i+=4;
//                data[0] = (i+4) as i64;
//                i = data[0] as usize;
            }

            if (data[0] == 19690720){
                println!("We found it! Noun is {} and verb is {}", noun, verb);
                break;
            } else {
//                println!("Tried {} and {}, didn't work", noun, verb);
            }
        }
    }



    return Ok(());
}

/// Read in the IntCode program in the given file, get (instructions, data)
fn readIntcodeProgram(f: &File) -> Result<(), String> {
    let buffer = BufReader::new(f);

    for line in buffer.lines() {
        return parseInstructionsAndData(line.unwrap());
    }

    return Err(String::from("Less the one line in data file"));
}

////! Runs the given program and returns the state of data after running
//fn runIntCodeProgram(instructions: Vec<Instruction>, initial_data: Vec<i64>) -> Vec<i64> {
//    let mut data = initial_data;
//
//    for instruction in instructions {
//        match instruction.opcode {
//            OpCode::Add => {
//
//            }
//        }
//    }
//
//    return data;
//}

fn main() -> Result<(), String>{
    let filepath = "src/bin/day_2/part_1_test_data";
    let file = File::open(filepath);

    match file {
        Err(e) => Err(String::from("FAILED TO READ FILE!")),
        Ok(f) => {
            readIntcodeProgram(&f);
            Ok(())
        }
    }
}
