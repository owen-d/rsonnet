use k8s_openapi::api::core::v1 as api;

fn idk() {
    let pod: api::PodSpec = Default::default();
    let p2: api::PodSpec = json!({"replicas": 2})?;

    println!("{:#?}", pod);
    println!("{:#?}", p2);
}
