fn main() {
    let instance = wgpu::Instance::new(&Default::default());
    let adapter = pollster::block_on(instance.request_adapter(&Default::default())).unwrap();
    let result = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        required_features: wgpu::Features::SHADER_F16,
        ..Default::default()
    })).unwrap();

    // To send a single string as the response body, use `res::respond`.
    println!("result: {result:#?}\n");
}
