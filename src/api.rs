use rocket::response::content;
use rocket::Request;
use rocket::State;
use rocket_contrib::json::Json;
mod cgminer;

pub struct CgminerConfig {
    pub addr: String,
    pub timeout: i64,
}

#[derive(Debug, Responder)]
#[response(status = 500, content_type = "json")]
pub struct ResponseError {
    response: content::Json<String>,
}

fn cmd(
    cgminer_config: &CgminerConfig,
    command: &str,
) -> Result<content::Json<String>, ResponseError> {
    match cgminer::cmd(&cgminer_config.addr, cgminer_config.timeout, command) {
        Ok(response) => Ok(content::Json(response)),
        Err(error) => Err(ResponseError {
            response: content::Json(json!({ "error": error }).to_string()),
        }),
    }
}

fn cmdp(
    cgminer_config: &CgminerConfig,
    command: &str,
    parameter: &str,
) -> Result<content::Json<String>, ResponseError> {
    match cgminer::cmdp(
        &cgminer_config.addr,
        cgminer_config.timeout,
        command,
        parameter,
    ) {
        Ok(response) => Ok(content::Json(response)),
        Err(error) => Err(ResponseError {
            response: content::Json(json!({ "error": error }).to_string()),
        }),
    }
}

#[get("/version")]
pub fn version(
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    cmd(&cgminer_config, "version")
}

#[get("/config")]
pub fn config(
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    cmd(&cgminer_config, "config")
}

#[get("/summary")]
pub fn summary(
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    cmd(&cgminer_config, "summary")
}

#[get("/devs")]
pub fn devs(cgminer_config: State<CgminerConfig>) -> Result<content::Json<String>, ResponseError> {
    cmd(&cgminer_config, "devs")
}

#[get("/devdetails")]
pub fn devdetails(
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    cmd(&cgminer_config, "devdetails")
}

#[get("/stats")]
pub fn stats(cgminer_config: State<CgminerConfig>) -> Result<content::Json<String>, ResponseError> {
    cmd(&cgminer_config, "stats")
}

#[get("/coin")]
pub fn coin(cgminer_config: State<CgminerConfig>) -> Result<content::Json<String>, ResponseError> {
    cmd(&cgminer_config, "coin")
}

#[get("/usbstats")]
pub fn usbstats(
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    cmd(&cgminer_config, "usbstats")
}

#[get("/lcd")]
pub fn lcd(cgminer_config: State<CgminerConfig>) -> Result<content::Json<String>, ResponseError> {
    cmd(&cgminer_config, "lcd")
}

#[get("/notify")]
pub fn notify(
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    cmd(&cgminer_config, "notify")
}

#[get("/privileged")]
pub fn privileged(
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    cmd(&cgminer_config, "privileged")
}

#[get("/lockstats")]
pub fn lockstats(
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    cmd(&cgminer_config, "lockstats")
}

#[get("/")]
pub fn pools(cgminer_config: State<CgminerConfig>) -> Result<content::Json<String>, ResponseError> {
    cmd(&cgminer_config, "pools")
}

#[put("/<id>/switchto")]
pub fn switchpool(
    id: u32,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{}", id).to_string();
    cmdp(&cgminer_config, "switchpool", &parameter)
}

#[put("/<id>/enable")]
pub fn enablepool(
    id: u32,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{}", id).to_string();
    cmdp(&cgminer_config, "enablepool", &parameter)
}

#[put("/<id>/disable")]
pub fn disablepool(
    id: u32,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{}", id).to_string();
    cmdp(&cgminer_config, "disablepool", &parameter)
}

#[derive(Deserialize)]
pub struct Pool {
    url: String,
    username: String,
    password: String,
}

#[post("/", data = "<pool>")]
pub fn addpool(
    pool: Json<Pool>,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{},{},{}", pool.url, pool.username, pool.password).to_string();
    cmdp(&cgminer_config, "addpool", &parameter)
}

#[delete("/<id>")]
pub fn removepool(
    id: u32,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{}", id).to_string();
    cmdp(&cgminer_config, "removepool", &parameter)
}

#[derive(Deserialize)]
pub struct PoolQuota {
    value: u32,
}

#[put("/<id>/quota", data = "<pool_quota>")]
pub fn poolquota(
    id: u32,
    pool_quota: Json<PoolQuota>,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{},{}", id, pool_quota.value).to_string();
    cmdp(&cgminer_config, "poolquota", &parameter)
}

#[get("/<id>")]
pub fn pga(
    id: u32,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{}", id).to_string();
    cmdp(&cgminer_config, "pga", &parameter)
}

#[get("/count")]
pub fn pgacount(
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    cmd(&cgminer_config, "pgacount")
}

#[put("/<id>/enable")]
pub fn pgaenable(
    id: u32,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{}", id).to_string();
    cmdp(&cgminer_config, "pgaenable", &parameter)
}

#[put("/<id>/disable")]
pub fn pgadisable(
    id: u32,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{}", id).to_string();
    cmdp(&cgminer_config, "pgadisable", &parameter)
}

#[get("/<id>/identify")]
pub fn pgaidentify(
    id: u32,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{}", id).to_string();
    cmdp(&cgminer_config, "pgaidentify", &parameter)
}

#[derive(Deserialize)]
pub struct PgaSetting {
    option: String,
    value: String,
}

#[put("/<id>", data = "<pga_setting>")]
pub fn pgaset(
    id: u32,
    pga_setting: Json<PgaSetting>,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{},{},{}", id, pga_setting.option, pga_setting.value).to_string();
    cmdp(&cgminer_config, "pgaset", &parameter)
}

#[get("/<id>")]
pub fn asc(
    id: u32,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{}", id).to_string();
    cmdp(&cgminer_config, "asc", &parameter)
}

#[get("/count")]
pub fn asccount(
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    cmd(&cgminer_config, "asccount")
}

#[put("/<id>/enable")]
pub fn ascenable(
    id: u32,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{}", id).to_string();
    cmdp(&cgminer_config, "ascenable", &parameter)
}

#[put("/<id>/disable")]
pub fn ascdisable(
    id: u32,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{}", id).to_string();
    cmdp(&cgminer_config, "ascdisable", &parameter)
}

#[get("/<id>/identify")]
pub fn ascidentify(
    id: u32,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{}", id).to_string();
    cmdp(&cgminer_config, "ascidentify", &parameter)
}

#[derive(Deserialize)]
pub struct AscSetting {
    option: String,
    value: String,
}

#[put("/<id>", data = "<asc_setting>")]
pub fn ascset(
    id: u32,
    asc_setting: Json<AscSetting>,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{},{},{}", id, asc_setting.option, asc_setting.value).to_string();
    cmdp(&cgminer_config, "ascset", &parameter)
}

#[put("/restart")]
pub fn restart(
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    cmd(&cgminer_config, "restart")
}

#[put("/check/<command>")]
pub fn check(
    command: String,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    cmdp(&cgminer_config, "check", &command)
}

#[derive(Deserialize)]
pub struct DebugSetting {
    name: String,
    value: bool,
}

#[put("/debug", data = "<debug_setting>")]
pub fn debug(
    debug_setting: Json<DebugSetting>,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{}={}", debug_setting.name, debug_setting.value).to_string();
    cmdp(&cgminer_config, "debug", &parameter)
}

#[derive(Deserialize)]
pub struct ZeroSetting {
    which: String,
    value: bool,
}

#[put("/zero", data = "<zero_setting>")]
pub fn zero(
    zero_setting: Json<ZeroSetting>,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{},{}", zero_setting.which, zero_setting.value).to_string();
    cmdp(&cgminer_config, "zero", &parameter)
}

#[derive(Deserialize)]
pub struct HotplugSetting {
    value: i64,
}

#[put("/hotplug", data = "<hotplug_setting>")]
pub fn hotplug(
    hotplug_setting: Json<HotplugSetting>,
    cgminer_config: State<CgminerConfig>,
) -> Result<content::Json<String>, ResponseError> {
    let parameter = format!("{}", hotplug_setting.value).to_string();
    cmdp(&cgminer_config, "hotplug", &parameter)
}

#[catch(404)]
pub fn not_found(req: &Request) -> content::Json<String> {
    content::Json(
        json!({
            "error":
                format!(
                    "Look elsewhere, perhaps? No matching route for uri={}",
                    req.uri()
                )
        })
        .to_string(),
    )
}

#[catch(500)]
pub fn internal_server_error(_req: &Request) -> content::Json<String> {
    content::Json(
        json!({
            "error":"Internal server error 🤖"
        })
        .to_string(),
    )
}

#[catch(422)]
pub fn unprocessable_entity(_req: &Request) -> content::Json<String> {
    content::Json(
        json!({
            "error":"The request was well-formed but was unable to be followed due to semantic errors."
        })
        .to_string(),
    )
}
