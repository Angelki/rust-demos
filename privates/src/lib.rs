mod outermost {
    pub fn middle_function() {}
    pub fn middle_secret_function() {}
    pub mod inside {
        pub fn inner_function() {}
        pub fn secret_function() {}
    }
}
pub fn try_me() {
    outermost::middle_function();
    // pub
    outermost::middle_secret_function();
    // pub
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
