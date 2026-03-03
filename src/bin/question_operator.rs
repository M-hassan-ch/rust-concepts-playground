fn f1() -> Result<u32, String> {
    Ok(1)
    // Err("f1 failed".to_string())
}

fn f2() -> Result<u32, String> {
    Ok(2)
    // Err("f2 failed".to_string())
}

// manually matching return values
fn f_match() -> Result<u32, String> {
    let value_f1 = f1();
    let value_f2 = f2();

    let result_f1 = match value_f1 {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let result_f2 = match value_f2 {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    Ok(result_f1 + result_f2)
}

// using question operator instead of manually matching return values
fn f_question() -> Result<u32, String>{
    let result_f1 = f1()?;
    let result_f2 = f2()?;

    Ok(result_f1 + result_f2)
}

fn main() -> Result<(), String> {
    // manual matching approach
    let result_match = f_match();
    match result_match {
        Ok(val) => println!("Result calling f_match: {val}"),
        Err(err) => println!("Error while calling f_match: {err}")
    }
    
    // using question operator
    let result_question = f_question();
    match result_question {
        Ok(val) => println!("Result calling f_question: {val}"),
        Err(err) => println!("Error while calling f_question: {err}")
    }

    Ok(())
}
