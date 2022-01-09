#[doc = "Register `INTERRUPT` reader"]
pub struct R(crate::R<INTERRUPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERRUPT` writer"]
pub struct W(crate::W<INTERRUPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTERRUPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SM_MULT_WR_INTR` reader - This bit is set when an FFE tries to write to more that one FIFO simultaneously. The FIFO PUSH value must be one hot, with only one pushd asserted."]
pub struct SM_MULT_WR_INTR_R(crate::FieldReader<bool, bool>);
impl SM_MULT_WR_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM_MULT_WR_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM_MULT_WR_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM_MULT_WR_INTR` writer - This bit is set when an FFE tries to write to more that one FIFO simultaneously. The FIFO PUSH value must be one hot, with only one pushd asserted."]
pub struct SM_MULT_WR_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_MULT_WR_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `PFE0_OVERRUN` reader - This bit is set when the FFE does not complete the algorithm by the time the next sample period begins. This bit can only be cleared by issuing a device reset (software reset, or global reset via the Reset pin)."]
pub struct PFE0_OVERRUN_R(crate::FieldReader<bool, bool>);
impl PFE0_OVERRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PFE0_OVERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PFE0_OVERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PFE0_OVERRUN` writer - This bit is set when the FFE does not complete the algorithm by the time the next sample period begins. This bit can only be cleared by issuing a device reset (software reset, or global reset via the Reset pin)."]
pub struct PFE0_OVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> PFE0_OVERRUN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `FFE0_SM1_OBERRUN` reader - This bit is set when the SM does not complete the algorithm by the time the next sample period begins. This bit can only be cleared by issuing a device reset (software reset, or global reset via the Reset pin)."]
pub struct FFE0_SM1_OBERRUN_R(crate::FieldReader<bool, bool>);
impl FFE0_SM1_OBERRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_SM1_OBERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_SM1_OBERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_SM1_OBERRUN` writer - This bit is set when the SM does not complete the algorithm by the time the next sample period begins. This bit can only be cleared by issuing a device reset (software reset, or global reset via the Reset pin)."]
pub struct FFE0_SM1_OBERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_SM1_OBERRUN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `FFE0_SM0_OBERRUN` reader - This bit is set when the SM does not complete the algorithm by the time the next sample period begins. This bit can only be cleared by issuing a device reset (software reset, or global reset via the Reset pin)."]
pub struct FFE0_SM0_OBERRUN_R(crate::FieldReader<bool, bool>);
impl FFE0_SM0_OBERRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_SM0_OBERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_SM0_OBERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_SM0_OBERRUN` writer - This bit is set when the SM does not complete the algorithm by the time the next sample period begins. This bit can only be cleared by issuing a device reset (software reset, or global reset via the Reset pin)."]
pub struct FFE0_SM0_OBERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_SM0_OBERRUN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `I2C_MS_1_ERROR` reader - This bit is set when the I2C Master receives a NACK when transmitting a device address. The I2C Master is used by the Sensor Manager to retrieve sensor data."]
pub struct I2C_MS_1_ERROR_R(crate::FieldReader<bool, bool>);
impl I2C_MS_1_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_MS_1_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_MS_1_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_MS_1_ERROR` writer - This bit is set when the I2C Master receives a NACK when transmitting a device address. The I2C Master is used by the Sensor Manager to retrieve sensor data."]
pub struct I2C_MS_1_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MS_1_ERROR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `I2C_MS_0_ERROR` reader - This bit is set when the I2C Master receives a NACK when transmitting a device address. The I2C Master is used by the Sensor Manager to retrieve sensor data."]
pub struct I2C_MS_0_ERROR_R(crate::FieldReader<bool, bool>);
impl I2C_MS_0_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_MS_0_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_MS_0_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_MS_0_ERROR` writer - This bit is set when the I2C Master receives a NACK when transmitting a device address. The I2C Master is used by the Sensor Manager to retrieve sensor data."]
pub struct I2C_MS_0_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MS_0_ERROR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CM_8k_LP_INTR` reader - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
pub struct CM_8K_LP_INTR_R(crate::FieldReader<bool, bool>);
impl CM_8K_LP_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CM_8K_LP_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CM_8K_LP_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM_8k_LP_INTR` writer - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
pub struct CM_8K_LP_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_8K_LP_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DM0_LP_INTR` reader - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
pub struct DM0_LP_INTR_R(crate::FieldReader<bool, bool>);
impl DM0_LP_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM0_LP_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM0_LP_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM0_LP_INTR` writer - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
pub struct DM0_LP_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DM0_LP_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `DM1_LP_INTR` reader - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
pub struct DM1_LP_INTR_R(crate::FieldReader<bool, bool>);
impl DM1_LP_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM1_LP_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM1_LP_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM1_LP_INTR` writer - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
pub struct DM1_LP_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DM1_LP_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SM0_LP_INTR` reader - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
pub struct SM0_LP_INTR_R(crate::FieldReader<bool, bool>);
impl SM0_LP_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM0_LP_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM0_LP_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM0_LP_INTR` writer - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
pub struct SM0_LP_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0_LP_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SM1_LP_INTR` reader - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
pub struct SM1_LP_INTR_R(crate::FieldReader<bool, bool>);
impl SM1_LP_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM1_LP_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM1_LP_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM1_LP_INTR` writer - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
pub struct SM1_LP_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_LP_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `FFE0_BP_MATCH_INTR` reader - This bit is set when there is a break point match in FFE0"]
pub struct FFE0_BP_MATCH_INTR_R(crate::FieldReader<bool, bool>);
impl FFE0_BP_MATCH_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_BP_MATCH_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_BP_MATCH_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_BP_MATCH_INTR` writer - This bit is set when there is a break point match in FFE0"]
pub struct FFE0_BP_MATCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_BP_MATCH_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `FFE1_OVERRUN` reader - This bit is set when the FFE does not complete the algorithm by the time the next sample period begins."]
pub struct FFE1_OVERRUN_R(crate::FieldReader<bool, bool>);
impl FFE1_OVERRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE1_OVERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE1_OVERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE1_OVERRUN` writer - This bit is set when the FFE does not complete the algorithm by the time the next sample period begins."]
pub struct FFE1_OVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE1_OVERRUN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `PKFB_OVF_INTR` reader - This bit is set when the FFE pushes to the PKFB causing an overflow"]
pub struct PKFB_OVF_INTR_R(crate::FieldReader<bool, bool>);
impl PKFB_OVF_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PKFB_OVF_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKFB_OVF_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKFB_OVF_INTR` writer - This bit is set when the FFE pushes to the PKFB causing an overflow"]
pub struct PKFB_OVF_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PKFB_OVF_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `SM0_BP_MATCH_INTR` reader - This bit is set when there is a break point match in SM0"]
pub struct SM0_BP_MATCH_INTR_R(crate::FieldReader<bool, bool>);
impl SM0_BP_MATCH_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM0_BP_MATCH_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM0_BP_MATCH_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM0_BP_MATCH_INTR` writer - This bit is set when there is a break point match in SM0"]
pub struct SM0_BP_MATCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0_BP_MATCH_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `SM1_BP_MATCH_INTR` reader - This bit is set when there is a break point match in SM1"]
pub struct SM1_BP_MATCH_INTR_R(crate::FieldReader<bool, bool>);
impl SM1_BP_MATCH_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM1_BP_MATCH_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM1_BP_MATCH_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM1_BP_MATCH_INTR` writer - This bit is set when there is a break point match in SM1"]
pub struct SM1_BP_MATCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_BP_MATCH_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `SPI_MS_INTR` reader - This bit is set when there is an interrupt request from SPI_MS for sensor"]
pub struct SPI_MS_INTR_R(crate::FieldReader<bool, bool>);
impl SPI_MS_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_MS_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_MS_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_MS_INTR` writer - This bit is set when there is an interrupt request from SPI_MS for sensor"]
pub struct SPI_MS_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MS_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CM_2k_LP_INTR` reader - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
pub struct CM_2K_LP_INTR_R(crate::FieldReader<bool, bool>);
impl CM_2K_LP_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CM_2K_LP_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CM_2K_LP_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM_2k_LP_INTR` writer - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
pub struct CM_2K_LP_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_2K_LP_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `DM2_LP_INTR` reader - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
pub struct DM2_LP_INTR_R(crate::FieldReader<bool, bool>);
impl DM2_LP_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM2_LP_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM2_LP_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM2_LP_INTR` writer - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
pub struct DM2_LP_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DM2_LP_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `DM3_LP_INTR` reader - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
pub struct DM3_LP_INTR_R(crate::FieldReader<bool, bool>);
impl DM3_LP_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM3_LP_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM3_LP_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM3_LP_INTR` writer - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
pub struct DM3_LP_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DM3_LP_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `ahbm_bus_error_INTR` reader - This bit is set when there is a bus error on the AHB bus (HRESP=1)."]
pub struct AHBM_BUS_ERROR_INTR_R(crate::FieldReader<bool, bool>);
impl AHBM_BUS_ERROR_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBM_BUS_ERROR_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBM_BUS_ERROR_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ahbm_bus_error_INTR` writer - This bit is set when there is a bus error on the AHB bus (HRESP=1)."]
pub struct AHBM_BUS_ERROR_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBM_BUS_ERROR_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This bit is set when an FFE tries to write to more that one FIFO simultaneously. The FIFO PUSH value must be one hot, with only one pushd asserted."]
    #[inline(always)]
    pub fn sm_mult_wr_intr(&self) -> SM_MULT_WR_INTR_R {
        SM_MULT_WR_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is set when the FFE does not complete the algorithm by the time the next sample period begins. This bit can only be cleared by issuing a device reset (software reset, or global reset via the Reset pin)."]
    #[inline(always)]
    pub fn pfe0_overrun(&self) -> PFE0_OVERRUN_R {
        PFE0_OVERRUN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit is set when the SM does not complete the algorithm by the time the next sample period begins. This bit can only be cleared by issuing a device reset (software reset, or global reset via the Reset pin)."]
    #[inline(always)]
    pub fn ffe0_sm1_oberrun(&self) -> FFE0_SM1_OBERRUN_R {
        FFE0_SM1_OBERRUN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit is set when the SM does not complete the algorithm by the time the next sample period begins. This bit can only be cleared by issuing a device reset (software reset, or global reset via the Reset pin)."]
    #[inline(always)]
    pub fn ffe0_sm0_oberrun(&self) -> FFE0_SM0_OBERRUN_R {
        FFE0_SM0_OBERRUN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit is set when the I2C Master receives a NACK when transmitting a device address. The I2C Master is used by the Sensor Manager to retrieve sensor data."]
    #[inline(always)]
    pub fn i2c_ms_1_error(&self) -> I2C_MS_1_ERROR_R {
        I2C_MS_1_ERROR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit is set when the I2C Master receives a NACK when transmitting a device address. The I2C Master is used by the Sensor Manager to retrieve sensor data."]
    #[inline(always)]
    pub fn i2c_ms_0_error(&self) -> I2C_MS_0_ERROR_R {
        I2C_MS_0_ERROR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
    #[inline(always)]
    pub fn cm_8k_lp_intr(&self) -> CM_8K_LP_INTR_R {
        CM_8K_LP_INTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
    #[inline(always)]
    pub fn dm0_lp_intr(&self) -> DM0_LP_INTR_R {
        DM0_LP_INTR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
    #[inline(always)]
    pub fn dm1_lp_intr(&self) -> DM1_LP_INTR_R {
        DM1_LP_INTR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
    #[inline(always)]
    pub fn sm0_lp_intr(&self) -> SM0_LP_INTR_R {
        SM0_LP_INTR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
    #[inline(always)]
    pub fn sm1_lp_intr(&self) -> SM1_LP_INTR_R {
        SM1_LP_INTR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This bit is set when there is a break point match in FFE0"]
    #[inline(always)]
    pub fn ffe0_bp_match_intr(&self) -> FFE0_BP_MATCH_INTR_R {
        FFE0_BP_MATCH_INTR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This bit is set when the FFE does not complete the algorithm by the time the next sample period begins."]
    #[inline(always)]
    pub fn ffe1_overrun(&self) -> FFE1_OVERRUN_R {
        FFE1_OVERRUN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This bit is set when the FFE pushes to the PKFB causing an overflow"]
    #[inline(always)]
    pub fn pkfb_ovf_intr(&self) -> PKFB_OVF_INTR_R {
        PKFB_OVF_INTR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This bit is set when there is a break point match in SM0"]
    #[inline(always)]
    pub fn sm0_bp_match_intr(&self) -> SM0_BP_MATCH_INTR_R {
        SM0_BP_MATCH_INTR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - This bit is set when there is a break point match in SM1"]
    #[inline(always)]
    pub fn sm1_bp_match_intr(&self) -> SM1_BP_MATCH_INTR_R {
        SM1_BP_MATCH_INTR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - This bit is set when there is an interrupt request from SPI_MS for sensor"]
    #[inline(always)]
    pub fn spi_ms_intr(&self) -> SPI_MS_INTR_R {
        SPI_MS_INTR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
    #[inline(always)]
    pub fn cm_2k_lp_intr(&self) -> CM_2K_LP_INTR_R {
        CM_2K_LP_INTR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
    #[inline(always)]
    pub fn dm2_lp_intr(&self) -> DM2_LP_INTR_R {
        DM2_LP_INTR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
    #[inline(always)]
    pub fn dm3_lp_intr(&self) -> DM3_LP_INTR_R {
        DM3_LP_INTR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - This bit is set when there is a bus error on the AHB bus (HRESP=1)."]
    #[inline(always)]
    pub fn ahbm_bus_error_intr(&self) -> AHBM_BUS_ERROR_INTR_R {
        AHBM_BUS_ERROR_INTR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set when an FFE tries to write to more that one FIFO simultaneously. The FIFO PUSH value must be one hot, with only one pushd asserted."]
    #[inline(always)]
    pub fn sm_mult_wr_intr(&mut self) -> SM_MULT_WR_INTR_W {
        SM_MULT_WR_INTR_W { w: self }
    }
    #[doc = "Bit 1 - This bit is set when the FFE does not complete the algorithm by the time the next sample period begins. This bit can only be cleared by issuing a device reset (software reset, or global reset via the Reset pin)."]
    #[inline(always)]
    pub fn pfe0_overrun(&mut self) -> PFE0_OVERRUN_W {
        PFE0_OVERRUN_W { w: self }
    }
    #[doc = "Bit 4 - This bit is set when the SM does not complete the algorithm by the time the next sample period begins. This bit can only be cleared by issuing a device reset (software reset, or global reset via the Reset pin)."]
    #[inline(always)]
    pub fn ffe0_sm1_oberrun(&mut self) -> FFE0_SM1_OBERRUN_W {
        FFE0_SM1_OBERRUN_W { w: self }
    }
    #[doc = "Bit 5 - This bit is set when the SM does not complete the algorithm by the time the next sample period begins. This bit can only be cleared by issuing a device reset (software reset, or global reset via the Reset pin)."]
    #[inline(always)]
    pub fn ffe0_sm0_oberrun(&mut self) -> FFE0_SM0_OBERRUN_W {
        FFE0_SM0_OBERRUN_W { w: self }
    }
    #[doc = "Bit 6 - This bit is set when the I2C Master receives a NACK when transmitting a device address. The I2C Master is used by the Sensor Manager to retrieve sensor data."]
    #[inline(always)]
    pub fn i2c_ms_1_error(&mut self) -> I2C_MS_1_ERROR_W {
        I2C_MS_1_ERROR_W { w: self }
    }
    #[doc = "Bit 7 - This bit is set when the I2C Master receives a NACK when transmitting a device address. The I2C Master is used by the Sensor Manager to retrieve sensor data."]
    #[inline(always)]
    pub fn i2c_ms_0_error(&mut self) -> I2C_MS_0_ERROR_W {
        I2C_MS_0_ERROR_W { w: self }
    }
    #[doc = "Bit 8 - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
    #[inline(always)]
    pub fn cm_8k_lp_intr(&mut self) -> CM_8K_LP_INTR_W {
        CM_8K_LP_INTR_W { w: self }
    }
    #[doc = "Bit 9 - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
    #[inline(always)]
    pub fn dm0_lp_intr(&mut self) -> DM0_LP_INTR_W {
        DM0_LP_INTR_W { w: self }
    }
    #[doc = "Bit 10 - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
    #[inline(always)]
    pub fn dm1_lp_intr(&mut self) -> DM1_LP_INTR_W {
        DM1_LP_INTR_W { w: self }
    }
    #[doc = "Bit 11 - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
    #[inline(always)]
    pub fn sm0_lp_intr(&mut self) -> SM0_LP_INTR_W {
        SM0_LP_INTR_W { w: self }
    }
    #[doc = "Bit 12 - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
    #[inline(always)]
    pub fn sm1_lp_intr(&mut self) -> SM1_LP_INTR_W {
        SM1_LP_INTR_W { w: self }
    }
    #[doc = "Bit 13 - This bit is set when there is a break point match in FFE0"]
    #[inline(always)]
    pub fn ffe0_bp_match_intr(&mut self) -> FFE0_BP_MATCH_INTR_W {
        FFE0_BP_MATCH_INTR_W { w: self }
    }
    #[doc = "Bit 14 - This bit is set when the FFE does not complete the algorithm by the time the next sample period begins."]
    #[inline(always)]
    pub fn ffe1_overrun(&mut self) -> FFE1_OVERRUN_W {
        FFE1_OVERRUN_W { w: self }
    }
    #[doc = "Bit 15 - This bit is set when the FFE pushes to the PKFB causing an overflow"]
    #[inline(always)]
    pub fn pkfb_ovf_intr(&mut self) -> PKFB_OVF_INTR_W {
        PKFB_OVF_INTR_W { w: self }
    }
    #[doc = "Bit 16 - This bit is set when there is a break point match in SM0"]
    #[inline(always)]
    pub fn sm0_bp_match_intr(&mut self) -> SM0_BP_MATCH_INTR_W {
        SM0_BP_MATCH_INTR_W { w: self }
    }
    #[doc = "Bit 17 - This bit is set when there is a break point match in SM1"]
    #[inline(always)]
    pub fn sm1_bp_match_intr(&mut self) -> SM1_BP_MATCH_INTR_W {
        SM1_BP_MATCH_INTR_W { w: self }
    }
    #[doc = "Bit 18 - This bit is set when there is an interrupt request from SPI_MS for sensor"]
    #[inline(always)]
    pub fn spi_ms_intr(&mut self) -> SPI_MS_INTR_W {
        SPI_MS_INTR_W { w: self }
    }
    #[doc = "Bit 19 - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
    #[inline(always)]
    pub fn cm_2k_lp_intr(&mut self) -> CM_2K_LP_INTR_W {
        CM_2K_LP_INTR_W { w: self }
    }
    #[doc = "Bit 20 - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
    #[inline(always)]
    pub fn dm2_lp_intr(&mut self) -> DM2_LP_INTR_W {
        DM2_LP_INTR_W { w: self }
    }
    #[doc = "Bit 21 - This bit is set when there is an access to the memory while it is in low power (deep sleep or shut down)"]
    #[inline(always)]
    pub fn dm3_lp_intr(&mut self) -> DM3_LP_INTR_W {
        DM3_LP_INTR_W { w: self }
    }
    #[doc = "Bit 22 - This bit is set when there is a bus error on the AHB bus (HRESP=1)."]
    #[inline(always)]
    pub fn ahbm_bus_error_intr(&mut self) -> AHBM_BUS_ERROR_INTR_W {
        AHBM_BUS_ERROR_INTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Varied interrupt configurations\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt](index.html) module"]
pub struct INTERRUPT_SPEC;
impl crate::RegisterSpec for INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt::R](R) reader structure"]
impl crate::Readable for INTERRUPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt::W](W) writer structure"]
impl crate::Writable for INTERRUPT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERRUPT to value 0"]
impl crate::Resettable for INTERRUPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
