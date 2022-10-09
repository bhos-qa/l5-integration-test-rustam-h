pub fn get_repos() -> String
{
    let res = reqwest::blocking::get("https://60a21d3f745cd70017576092.mockapi.io/api/v1/repos").unwrap();
    let body = res.text().unwrap();
    println!("{}", body);

    body
}

pub fn get_branches() -> String
{
    let res = reqwest::blocking::get("https://60a21d3f745cd70017576092.mockapi.io/api/v1/repos/1/branches").unwrap();
    let body = res.text().unwrap();
    println!("{}", body);

    body
}

pub fn get_commits() -> String
{
    let res = reqwest::blocking::get("https://60a21d3f745cd70017576092.mockapi.io/api/v1/repos/1/branches/1/commits").unwrap();
    let body = res.text().unwrap();
    println!("{}", body);

    body
}