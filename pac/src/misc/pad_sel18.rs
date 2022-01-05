#[doc = "Register `PAD_SEL18` reader"]
pub struct R(crate::R<PAD_SEL18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_SEL18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_SEL18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_SEL18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_SEL18` writer"]
pub struct W(crate::W<PAD_SEL18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_SEL18_SPEC>;
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
impl From<crate::W<PAD_SEL18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_SEL18_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOP_BANK_0` reader - bit for top bank 0"]
pub struct TOP_BANK_0_R(crate::FieldReader<bool, bool>);
impl TOP_BANK_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOP_BANK_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_BANK_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOP_BANK_0` writer - bit for top bank 0"]
pub struct TOP_BANK_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_BANK_0_W<'a> {
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
#[doc = "Field `TOP_BANK_1` reader - bit for top bank 1"]
pub struct TOP_BANK_1_R(crate::FieldReader<bool, bool>);
impl TOP_BANK_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOP_BANK_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_BANK_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOP_BANK_1` writer - bit for top bank 1"]
pub struct TOP_BANK_1_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_BANK_1_W<'a> {
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
#[doc = "Field `BOTTOM_BANK_2` reader - bit for bottom bank 0"]
pub struct BOTTOM_BANK_2_R(crate::FieldReader<bool, bool>);
impl BOTTOM_BANK_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOTTOM_BANK_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOTTOM_BANK_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOTTOM_BANK_2` writer - bit for bottom bank 0"]
pub struct BOTTOM_BANK_2_W<'a> {
    w: &'a mut W,
}
impl<'a> BOTTOM_BANK_2_W<'a> {
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
#[doc = "Field `BOTTOM_BANK_3` reader - bit for bottom bank 1"]
pub struct BOTTOM_BANK_3_R(crate::FieldReader<bool, bool>);
impl BOTTOM_BANK_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOTTOM_BANK_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOTTOM_BANK_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOTTOM_BANK_3` writer - bit for bottom bank 1"]
pub struct BOTTOM_BANK_3_W<'a> {
    w: &'a mut W,
}
impl<'a> BOTTOM_BANK_3_W<'a> {
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
impl R {
    #[doc = "Bit 0 - bit for top bank 0"]
    #[inline(always)]
    pub fn top_bank_0(&self) -> TOP_BANK_0_R {
        TOP_BANK_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - bit for top bank 1"]
    #[inline(always)]
    pub fn top_bank_1(&self) -> TOP_BANK_1_R {
        TOP_BANK_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - bit for bottom bank 0"]
    #[inline(always)]
    pub fn bottom_bank_2(&self) -> BOTTOM_BANK_2_R {
        BOTTOM_BANK_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - bit for bottom bank 1"]
    #[inline(always)]
    pub fn bottom_bank_3(&self) -> BOTTOM_BANK_3_R {
        BOTTOM_BANK_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - bit for top bank 0"]
    #[inline(always)]
    pub fn top_bank_0(&mut self) -> TOP_BANK_0_W {
        TOP_BANK_0_W { w: self }
    }
    #[doc = "Bit 1 - bit for top bank 1"]
    #[inline(always)]
    pub fn top_bank_1(&mut self) -> TOP_BANK_1_W {
        TOP_BANK_1_W { w: self }
    }
    #[doc = "Bit 2 - bit for bottom bank 0"]
    #[inline(always)]
    pub fn bottom_bank_2(&mut self) -> BOTTOM_BANK_2_W {
        BOTTOM_BANK_2_W { w: self }
    }
    #[doc = "Bit 3 - bit for bottom bank 1"]
    #[inline(always)]
    pub fn bottom_bank_3(&mut self) -> BOTTOM_BANK_3_W {
        BOTTOM_BANK_3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select 1.8V for VCCIO for up to 4 banks. Write a 1 to a bank field to make the IO VCC = 1.8V\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_sel18](index.html) module"]
pub struct PAD_SEL18_SPEC;
impl crate::RegisterSpec for PAD_SEL18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_sel18::R](R) reader structure"]
impl crate::Readable for PAD_SEL18_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_sel18::W](W) writer structure"]
impl crate::Writable for PAD_SEL18_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_SEL18 to value 0"]
impl crate::Resettable for PAD_SEL18_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
