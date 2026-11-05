use sigmaos_neoasknot::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_neoasknot::Neoasknot::new("sigmaos-neoasknot").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-neoasknot"));
}
