#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Last sampled value"]
    pub adc_out: crate::Reg<adc_out::ADC_OUT_SPEC>,
    #[doc = "0x04 - Status"]
    pub adc_status: crate::Reg<adc_status::ADC_STATUS_SPEC>,
    #[doc = "0x08 - Control register"]
    pub adc_control: crate::Reg<adc_control::ADC_CONTROL_SPEC>,
}
#[doc = "ADC_OUT register accessor: an alias for `Reg<ADC_OUT_SPEC>`"]
pub type ADC_OUT = crate::Reg<adc_out::ADC_OUT_SPEC>;
#[doc = "Last sampled value"]
pub mod adc_out;
#[doc = "ADC_Status register accessor: an alias for `Reg<ADC_STATUS_SPEC>`"]
pub type ADC_STATUS = crate::Reg<adc_status::ADC_STATUS_SPEC>;
#[doc = "Status"]
pub mod adc_status;
#[doc = "ADC_Control register accessor: an alias for `Reg<ADC_CONTROL_SPEC>`"]
pub type ADC_CONTROL = crate::Reg<adc_control::ADC_CONTROL_SPEC>;
#[doc = "Control register"]
pub mod adc_control;
