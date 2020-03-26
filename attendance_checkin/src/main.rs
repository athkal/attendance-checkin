use reqwest;
use std::io;

fn read_line() -> String {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line.trim_end().to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //normal compile - cargo build
    //cross compile - cross build --target=x86_64-unknown-linux-gnu
    println!("enter user id: ");
    let user_id = read_line();
    println!("enter genesis password: ");
    let password = read_line();

    //need to set a user agent, or it gets blocked. Literally any user agent works
    let agent = reqwest::Client::builder()
        .cookie_store(true)
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:74.0) Gecko/20100101 Firefox/74.0",
        )
        .build()?;

    let mut genesis_checkin = GenesisCheckin { agent };
    genesis_checkin.visit_main_site().await?;
    genesis_checkin
        .login_to_site(&format!("{}@whrhs-stu.org", user_id), &password)
        .await?;
    genesis_checkin.send_attendance(&user_id).await?;

    println!("done");
    Ok(())
}

struct GenesisCheckin {
    agent: reqwest::Client,
}

impl GenesisCheckin {
    async fn visit_main_site(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.agent
            .get("https://parents.whrhs.org/genesis/sis/view?gohome=true")
            .send()
            .await?;
        Ok(())
    }

    async fn login_to_site(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let params = [("j_username", username), ("j_password", password)];

        let resp = self
            .agent
            .post("https://parents.whrhs.org/genesis/sis/j_security_check")
            .form(&params)
            .send()
            .await?;
        for cookie in resp.cookies() {
            println!("{}: {}", cookie.name(), cookie.value());
        }
        println!("attempted login");
        Ok(())
    }

    async fn send_attendance(&self, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let params = [
            (user_id, "on"),
            ("attendanceType", "present"),
            ("attendanceDate", ""),
            ("attendanceReturnDate", ""),
            ("fldReason", ""),
        ];
        let api_url = format!("https://parents.whrhs.org/genesis/parents?tab1=studentdata&tab2=attendance&tab3=notify&studentid={}&action=notifyOffice", user_id);

        self.agent.post(&api_url).form(&params).send().await?;
        Ok(())
    }
}