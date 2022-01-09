#[doc = "Register `SRAM_TEST_REG1` reader"]
pub struct R(crate::R<SRAM_TEST_REG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_TEST_REG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_TEST_REG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_TEST_REG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_TEST_REG1` writer"]
pub struct W(crate::W<SRAM_TEST_REG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_TEST_REG1_SPEC>;
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
impl From<crate::W<SRAM_TEST_REG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_TEST_REG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SM1_TEST1` reader - SM1_TEST1 control for FFE SRAM"]
pub struct SM1_TEST1_R(crate::FieldReader<bool, bool>);
impl SM1_TEST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM1_TEST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM1_TEST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM1_TEST1` writer - SM1_TEST1 control for FFE SRAM"]
pub struct SM1_TEST1_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_TEST1_W<'a> {
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
#[doc = "Field `SM1_RME` reader - SM1_RME control for FFE SRAM"]
pub struct SM1_RME_R(crate::FieldReader<bool, bool>);
impl SM1_RME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM1_RME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM1_RME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM1_RME` writer - SM1_RME control for FFE SRAM"]
pub struct SM1_RME_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_RME_W<'a> {
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
#[doc = "Field `SM1_RM` reader - SM1_RM control for FFE SRAM"]
pub struct SM1_RM_R(crate::FieldReader<u8, u8>);
impl SM1_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SM1_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM1_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM1_RM` writer - SM1_RM control for FFE SRAM"]
pub struct SM1_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
#[doc = "Field `SM0_TEST1` reader - SM0_TEST1 control for FFE SRAM"]
pub struct SM0_TEST1_R(crate::FieldReader<bool, bool>);
impl SM0_TEST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM0_TEST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM0_TEST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM0_TEST1` writer - SM0_TEST1 control for FFE SRAM"]
pub struct SM0_TEST1_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0_TEST1_W<'a> {
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
#[doc = "Field `SM0_RME` reader - SM0_RME control for FFE SRAM"]
pub struct SM0_RME_R(crate::FieldReader<bool, bool>);
impl SM0_RME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM0_RME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM0_RME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM0_RME` writer - SM0_RME control for FFE SRAM"]
pub struct SM0_RME_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0_RME_W<'a> {
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
#[doc = "Field `SM0_RM` reader - SM0_RM control for FFE SRAM"]
pub struct SM0_RM_R(crate::FieldReader<u8, u8>);
impl SM0_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SM0_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM0_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM0_RM` writer - SM0_RM control for FFE SRAM"]
pub struct SM0_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `CM8k_TEST1` reader - CM8k_TEST1 control for FFE SRAM"]
pub struct CM8K_TEST1_R(crate::FieldReader<bool, bool>);
impl CM8K_TEST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CM8K_TEST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CM8K_TEST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM8k_TEST1` writer - CM8k_TEST1 control for FFE SRAM"]
pub struct CM8K_TEST1_W<'a> {
    w: &'a mut W,
}
impl<'a> CM8K_TEST1_W<'a> {
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
#[doc = "Field `CM8k_RME` reader - CM8k_RME control for FFE SRAM"]
pub struct CM8K_RME_R(crate::FieldReader<bool, bool>);
impl CM8K_RME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CM8K_RME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CM8K_RME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM8k_RME` writer - CM8k_RME control for FFE SRAM"]
pub struct CM8K_RME_W<'a> {
    w: &'a mut W,
}
impl<'a> CM8K_RME_W<'a> {
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
#[doc = "Field `CM8k_RM` reader - CM8k_RM control for FFE SRAM"]
pub struct CM8K_RM_R(crate::FieldReader<u8, u8>);
impl CM8K_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CM8K_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CM8K_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM8k_RM` writer - CM8k_RM control for FFE SRAM"]
pub struct CM8K_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM8K_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 14)) | ((value as u32 & 0x0f) << 14);
        self.w
    }
}
#[doc = "Field `CM2k_TEST1` reader - CM2k_TEST1 control for FFE SRAM"]
pub struct CM2K_TEST1_R(crate::FieldReader<bool, bool>);
impl CM2K_TEST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CM2K_TEST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CM2K_TEST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM2k_TEST1` writer - CM2k_TEST1 control for FFE SRAM"]
pub struct CM2K_TEST1_W<'a> {
    w: &'a mut W,
}
impl<'a> CM2K_TEST1_W<'a> {
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
#[doc = "Field `CM2k_RME` reader - CM2k_RME control for FFE SRAM"]
pub struct CM2K_RME_R(crate::FieldReader<bool, bool>);
impl CM2K_RME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CM2K_RME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CM2K_RME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM2k_RME` writer - CM2k_RME control for FFE SRAM"]
pub struct CM2K_RME_W<'a> {
    w: &'a mut W,
}
impl<'a> CM2K_RME_W<'a> {
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
#[doc = "Field `CM2k_RM` reader - CM2k_RM control for FFE SRAM"]
pub struct CM2K_RM_R(crate::FieldReader<u8, u8>);
impl CM2K_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CM2K_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CM2K_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM2k_RM` writer - CM2k_RM control for FFE SRAM"]
pub struct CM2K_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM2K_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `DM0_TEST1` reader - DM0_TEST1 control for FFE SRAM"]
pub struct DM0_TEST1_R(crate::FieldReader<bool, bool>);
impl DM0_TEST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM0_TEST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM0_TEST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM0_TEST1` writer - DM0_TEST1 control for FFE SRAM"]
pub struct DM0_TEST1_W<'a> {
    w: &'a mut W,
}
impl<'a> DM0_TEST1_W<'a> {
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
            (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `DM0_RME` reader - DM0_RME control for FFE SRAM"]
pub struct DM0_RME_R(crate::FieldReader<bool, bool>);
impl DM0_RME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM0_RME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM0_RME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM0_RME` writer - DM0_RME control for FFE SRAM"]
pub struct DM0_RME_W<'a> {
    w: &'a mut W,
}
impl<'a> DM0_RME_W<'a> {
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
            (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `DM0_RM` reader - DM0_RM control for FFE SRAM"]
pub struct DM0_RM_R(crate::FieldReader<u8, u8>);
impl DM0_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DM0_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM0_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM0_RM` writer - DM0_RM control for FFE SRAM"]
pub struct DM0_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> DM0_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 26)) | ((value as u32 & 0x0f) << 26);
        self.w
    }
}
#[doc = "Field `DM1_TEST1` reader - DM1_TEST1 control for FFE SRAM"]
pub struct DM1_TEST1_R(crate::FieldReader<bool, bool>);
impl DM1_TEST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM1_TEST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM1_TEST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM1_TEST1` writer - DM1_TEST1 control for FFE SRAM"]
pub struct DM1_TEST1_W<'a> {
    w: &'a mut W,
}
impl<'a> DM1_TEST1_W<'a> {
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
            (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `DM1_RME` reader - DM1_RME control for FFE SRAM"]
pub struct DM1_RME_R(crate::FieldReader<bool, bool>);
impl DM1_RME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM1_RME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM1_RME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM1_RME` writer - DM1_RME control for FFE SRAM"]
pub struct DM1_RME_W<'a> {
    w: &'a mut W,
}
impl<'a> DM1_RME_W<'a> {
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
            (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SM1_TEST1 control for FFE SRAM"]
    #[inline(always)]
    pub fn sm1_test1(&self) -> SM1_TEST1_R {
        SM1_TEST1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SM1_RME control for FFE SRAM"]
    #[inline(always)]
    pub fn sm1_rme(&self) -> SM1_RME_R {
        SM1_RME_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - SM1_RM control for FFE SRAM"]
    #[inline(always)]
    pub fn sm1_rm(&self) -> SM1_RM_R {
        SM1_RM_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - SM0_TEST1 control for FFE SRAM"]
    #[inline(always)]
    pub fn sm0_test1(&self) -> SM0_TEST1_R {
        SM0_TEST1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SM0_RME control for FFE SRAM"]
    #[inline(always)]
    pub fn sm0_rme(&self) -> SM0_RME_R {
        SM0_RME_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - SM0_RM control for FFE SRAM"]
    #[inline(always)]
    pub fn sm0_rm(&self) -> SM0_RM_R {
        SM0_RM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - CM8k_TEST1 control for FFE SRAM"]
    #[inline(always)]
    pub fn cm8k_test1(&self) -> CM8K_TEST1_R {
        CM8K_TEST1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CM8k_RME control for FFE SRAM"]
    #[inline(always)]
    pub fn cm8k_rme(&self) -> CM8K_RME_R {
        CM8K_RME_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:17 - CM8k_RM control for FFE SRAM"]
    #[inline(always)]
    pub fn cm8k_rm(&self) -> CM8K_RM_R {
        CM8K_RM_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - CM2k_TEST1 control for FFE SRAM"]
    #[inline(always)]
    pub fn cm2k_test1(&self) -> CM2K_TEST1_R {
        CM2K_TEST1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CM2k_RME control for FFE SRAM"]
    #[inline(always)]
    pub fn cm2k_rme(&self) -> CM2K_RME_R {
        CM2K_RME_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - CM2k_RM control for FFE SRAM"]
    #[inline(always)]
    pub fn cm2k_rm(&self) -> CM2K_RM_R {
        CM2K_RM_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - DM0_TEST1 control for FFE SRAM"]
    #[inline(always)]
    pub fn dm0_test1(&self) -> DM0_TEST1_R {
        DM0_TEST1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DM0_RME control for FFE SRAM"]
    #[inline(always)]
    pub fn dm0_rme(&self) -> DM0_RME_R {
        DM0_RME_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:29 - DM0_RM control for FFE SRAM"]
    #[inline(always)]
    pub fn dm0_rm(&self) -> DM0_RM_R {
        DM0_RM_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - DM1_TEST1 control for FFE SRAM"]
    #[inline(always)]
    pub fn dm1_test1(&self) -> DM1_TEST1_R {
        DM1_TEST1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DM1_RME control for FFE SRAM"]
    #[inline(always)]
    pub fn dm1_rme(&self) -> DM1_RME_R {
        DM1_RME_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SM1_TEST1 control for FFE SRAM"]
    #[inline(always)]
    pub fn sm1_test1(&mut self) -> SM1_TEST1_W {
        SM1_TEST1_W { w: self }
    }
    #[doc = "Bit 1 - SM1_RME control for FFE SRAM"]
    #[inline(always)]
    pub fn sm1_rme(&mut self) -> SM1_RME_W {
        SM1_RME_W { w: self }
    }
    #[doc = "Bits 2:5 - SM1_RM control for FFE SRAM"]
    #[inline(always)]
    pub fn sm1_rm(&mut self) -> SM1_RM_W {
        SM1_RM_W { w: self }
    }
    #[doc = "Bit 6 - SM0_TEST1 control for FFE SRAM"]
    #[inline(always)]
    pub fn sm0_test1(&mut self) -> SM0_TEST1_W {
        SM0_TEST1_W { w: self }
    }
    #[doc = "Bit 7 - SM0_RME control for FFE SRAM"]
    #[inline(always)]
    pub fn sm0_rme(&mut self) -> SM0_RME_W {
        SM0_RME_W { w: self }
    }
    #[doc = "Bits 8:11 - SM0_RM control for FFE SRAM"]
    #[inline(always)]
    pub fn sm0_rm(&mut self) -> SM0_RM_W {
        SM0_RM_W { w: self }
    }
    #[doc = "Bit 12 - CM8k_TEST1 control for FFE SRAM"]
    #[inline(always)]
    pub fn cm8k_test1(&mut self) -> CM8K_TEST1_W {
        CM8K_TEST1_W { w: self }
    }
    #[doc = "Bit 13 - CM8k_RME control for FFE SRAM"]
    #[inline(always)]
    pub fn cm8k_rme(&mut self) -> CM8K_RME_W {
        CM8K_RME_W { w: self }
    }
    #[doc = "Bits 14:17 - CM8k_RM control for FFE SRAM"]
    #[inline(always)]
    pub fn cm8k_rm(&mut self) -> CM8K_RM_W {
        CM8K_RM_W { w: self }
    }
    #[doc = "Bit 18 - CM2k_TEST1 control for FFE SRAM"]
    #[inline(always)]
    pub fn cm2k_test1(&mut self) -> CM2K_TEST1_W {
        CM2K_TEST1_W { w: self }
    }
    #[doc = "Bit 19 - CM2k_RME control for FFE SRAM"]
    #[inline(always)]
    pub fn cm2k_rme(&mut self) -> CM2K_RME_W {
        CM2K_RME_W { w: self }
    }
    #[doc = "Bits 20:23 - CM2k_RM control for FFE SRAM"]
    #[inline(always)]
    pub fn cm2k_rm(&mut self) -> CM2K_RM_W {
        CM2K_RM_W { w: self }
    }
    #[doc = "Bit 24 - DM0_TEST1 control for FFE SRAM"]
    #[inline(always)]
    pub fn dm0_test1(&mut self) -> DM0_TEST1_W {
        DM0_TEST1_W { w: self }
    }
    #[doc = "Bit 25 - DM0_RME control for FFE SRAM"]
    #[inline(always)]
    pub fn dm0_rme(&mut self) -> DM0_RME_W {
        DM0_RME_W { w: self }
    }
    #[doc = "Bits 26:29 - DM0_RM control for FFE SRAM"]
    #[inline(always)]
    pub fn dm0_rm(&mut self) -> DM0_RM_W {
        DM0_RM_W { w: self }
    }
    #[doc = "Bit 30 - DM1_TEST1 control for FFE SRAM"]
    #[inline(always)]
    pub fn dm1_test1(&mut self) -> DM1_TEST1_W {
        DM1_TEST1_W { w: self }
    }
    #[doc = "Bit 31 - DM1_RME control for FFE SRAM"]
    #[inline(always)]
    pub fn dm1_rme(&mut self) -> DM1_RME_W {
        DM1_RME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM test control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_test_reg1](index.html) module"]
pub struct SRAM_TEST_REG1_SPEC;
impl crate::RegisterSpec for SRAM_TEST_REG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_test_reg1::R](R) reader structure"]
impl crate::Readable for SRAM_TEST_REG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_test_reg1::W](W) writer structure"]
impl crate::Writable for SRAM_TEST_REG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAM_TEST_REG1 to value 0"]
impl crate::Resettable for SRAM_TEST_REG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
