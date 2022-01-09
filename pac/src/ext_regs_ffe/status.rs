#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SM0_BUSY` reader - This bit is set whenever the Sensor Manager is busy."]
pub struct SM0_BUSY_R(crate::FieldReader<bool, bool>);
impl SM0_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM0_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM0_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM1_BUSY` reader - This bit is set whenever the Sensor Manager is busy."]
pub struct SM1_BUSY_R(crate::FieldReader<bool, bool>);
impl SM1_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM1_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM1_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_BUSY` reader - This bit is set whenever the FFE0 is busy."]
pub struct FFE0_BUSY_R(crate::FieldReader<bool, bool>);
impl FFE0_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_BUSY` writer - This bit is set whenever the FFE0 is busy."]
pub struct FFE0_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_BUSY_W<'a> {
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
            (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `FFE1_BUSY` reader - This bit is set whenever the FFE1 is busy."]
pub struct FFE1_BUSY_R(crate::FieldReader<bool, bool>);
impl FFE1_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE1_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE1_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE1_BUSY` writer - This bit is set whenever the FFE1 is busy."]
pub struct FFE1_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE1_BUSY_W<'a> {
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
            (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `FFE0_BG_FLAG` reader - This is the ffe0 background thread status"]
pub struct FFE0_BG_FLAG_R(crate::FieldReader<bool, bool>);
impl FFE0_BG_FLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_BG_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_BG_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_BG_FLAG` writer - This is the ffe0 background thread status"]
pub struct FFE0_BG_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_BG_FLAG_W<'a> {
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
#[doc = "Field `FFE0_FG_FLAG` reader - This is the ffe0 background thread status"]
pub struct FFE0_FG_FLAG_R(crate::FieldReader<bool, bool>);
impl FFE0_FG_FLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_FG_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_FG_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_FG_FLAG` writer - This is the ffe0 background thread status"]
pub struct FFE0_FG_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_FG_FLAG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - This bit is set whenever the Sensor Manager is busy."]
    #[inline(always)]
    pub fn sm0_busy(&self) -> SM0_BUSY_R {
        SM0_BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is set whenever the Sensor Manager is busy."]
    #[inline(always)]
    pub fn sm1_busy(&self) -> SM1_BUSY_R {
        SM1_BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit is set whenever the FFE0 is busy."]
    #[inline(always)]
    pub fn ffe0_busy(&self) -> FFE0_BUSY_R {
        FFE0_BUSY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is set whenever the FFE1 is busy."]
    #[inline(always)]
    pub fn ffe1_busy(&self) -> FFE1_BUSY_R {
        FFE1_BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This is the ffe0 background thread status"]
    #[inline(always)]
    pub fn ffe0_bg_flag(&self) -> FFE0_BG_FLAG_R {
        FFE0_BG_FLAG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This is the ffe0 background thread status"]
    #[inline(always)]
    pub fn ffe0_fg_flag(&self) -> FFE0_FG_FLAG_R {
        FFE0_FG_FLAG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit is set whenever the FFE0 is busy."]
    #[inline(always)]
    pub fn ffe0_busy(&mut self) -> FFE0_BUSY_W {
        FFE0_BUSY_W { w: self }
    }
    #[doc = "Bit 3 - This bit is set whenever the FFE1 is busy."]
    #[inline(always)]
    pub fn ffe1_busy(&mut self) -> FFE1_BUSY_W {
        FFE1_BUSY_W { w: self }
    }
    #[doc = "Bit 4 - This is the ffe0 background thread status"]
    #[inline(always)]
    pub fn ffe0_bg_flag(&mut self) -> FFE0_BG_FLAG_W {
        FFE0_BG_FLAG_W { w: self }
    }
    #[doc = "Bit 5 - This is the ffe0 background thread status"]
    #[inline(always)]
    pub fn ffe0_fg_flag(&mut self) -> FFE0_FG_FLAG_W {
        FFE0_FG_FLAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FFE status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
