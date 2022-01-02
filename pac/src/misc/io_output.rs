#[doc = "Register `IO_OUTPUT` reader"]
pub struct R(crate::R<IO_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IO_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IO_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IO_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IO_OUTPUT` writer"]
pub struct W(crate::W<IO_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IO_OUTPUT_SPEC>;
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
impl From<crate::W<IO_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IO_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IO_0` reader - Read digital value of pin mapped to IO bit 0"]
pub struct IO_0_R(crate::FieldReader<bool, bool>);
impl IO_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_0` writer - Read digital value of pin mapped to IO bit 0"]
pub struct IO_0_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_0_W<'a> {
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
#[doc = "Field `IO_1` reader - Read digital value of pin mapped to IO bit 1"]
pub struct IO_1_R(crate::FieldReader<bool, bool>);
impl IO_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_1` writer - Read digital value of pin mapped to IO bit 1"]
pub struct IO_1_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `IO_2` reader - Read digital value of pin mapped to IO bit 2"]
pub struct IO_2_R(crate::FieldReader<bool, bool>);
impl IO_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_2` writer - Read digital value of pin mapped to IO bit 2"]
pub struct IO_2_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `IO_3` reader - Read digital value of pin mapped to IO bit 3"]
pub struct IO_3_R(crate::FieldReader<bool, bool>);
impl IO_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_3` writer - Read digital value of pin mapped to IO bit 3"]
pub struct IO_3_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `IO_4` reader - Read digital value of pin mapped to IO bit 4"]
pub struct IO_4_R(crate::FieldReader<bool, bool>);
impl IO_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_4` writer - Read digital value of pin mapped to IO bit 4"]
pub struct IO_4_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `IO_5` reader - Read digital value of pin mapped to IO bit 5"]
pub struct IO_5_R(crate::FieldReader<bool, bool>);
impl IO_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_5` writer - Read digital value of pin mapped to IO bit 5"]
pub struct IO_5_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `IO_6` reader - Read digital value of pin mapped to IO bit 6"]
pub struct IO_6_R(crate::FieldReader<bool, bool>);
impl IO_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_6` writer - Read digital value of pin mapped to IO bit 6"]
pub struct IO_6_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `IO_7` reader - Read digital value of pin mapped to IO bit 7"]
pub struct IO_7_R(crate::FieldReader<bool, bool>);
impl IO_7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_7` writer - Read digital value of pin mapped to IO bit 7"]
pub struct IO_7_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Read digital value of pin mapped to IO bit 0"]
    #[inline(always)]
    pub fn io_0(&self) -> IO_0_R {
        IO_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read digital value of pin mapped to IO bit 1"]
    #[inline(always)]
    pub fn io_1(&self) -> IO_1_R {
        IO_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read digital value of pin mapped to IO bit 2"]
    #[inline(always)]
    pub fn io_2(&self) -> IO_2_R {
        IO_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Read digital value of pin mapped to IO bit 3"]
    #[inline(always)]
    pub fn io_3(&self) -> IO_3_R {
        IO_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read digital value of pin mapped to IO bit 4"]
    #[inline(always)]
    pub fn io_4(&self) -> IO_4_R {
        IO_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Read digital value of pin mapped to IO bit 5"]
    #[inline(always)]
    pub fn io_5(&self) -> IO_5_R {
        IO_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read digital value of pin mapped to IO bit 6"]
    #[inline(always)]
    pub fn io_6(&self) -> IO_6_R {
        IO_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Read digital value of pin mapped to IO bit 7"]
    #[inline(always)]
    pub fn io_7(&self) -> IO_7_R {
        IO_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read digital value of pin mapped to IO bit 0"]
    #[inline(always)]
    pub fn io_0(&mut self) -> IO_0_W {
        IO_0_W { w: self }
    }
    #[doc = "Bit 1 - Read digital value of pin mapped to IO bit 1"]
    #[inline(always)]
    pub fn io_1(&mut self) -> IO_1_W {
        IO_1_W { w: self }
    }
    #[doc = "Bit 2 - Read digital value of pin mapped to IO bit 2"]
    #[inline(always)]
    pub fn io_2(&mut self) -> IO_2_W {
        IO_2_W { w: self }
    }
    #[doc = "Bit 3 - Read digital value of pin mapped to IO bit 3"]
    #[inline(always)]
    pub fn io_3(&mut self) -> IO_3_W {
        IO_3_W { w: self }
    }
    #[doc = "Bit 4 - Read digital value of pin mapped to IO bit 4"]
    #[inline(always)]
    pub fn io_4(&mut self) -> IO_4_W {
        IO_4_W { w: self }
    }
    #[doc = "Bit 5 - Read digital value of pin mapped to IO bit 5"]
    #[inline(always)]
    pub fn io_5(&mut self) -> IO_5_W {
        IO_5_W { w: self }
    }
    #[doc = "Bit 6 - Read digital value of pin mapped to IO bit 6"]
    #[inline(always)]
    pub fn io_6(&mut self) -> IO_6_W {
        IO_6_W { w: self }
    }
    #[doc = "Bit 7 - Read digital value of pin mapped to IO bit 7"]
    #[inline(always)]
    pub fn io_7(&mut self) -> IO_7_W {
        IO_7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Allows FW to drive the IO with the values specified in this register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io_output](index.html) module"]
pub struct IO_OUTPUT_SPEC;
impl crate::RegisterSpec for IO_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [io_output::R](R) reader structure"]
impl crate::Readable for IO_OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [io_output::W](W) writer structure"]
impl crate::Writable for IO_OUTPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IO_OUTPUT to value 0"]
impl crate::Resettable for IO_OUTPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
