use altbinutils::app::invoke_application;
use app_lse::app::LseApplication;

#[async_std::main]
async fn main() {
    invoke_application(LseApplication).await;
}
