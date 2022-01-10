#[doc = "Register `FB_INTR` reader"]
pub struct R(crate::R<FB_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FB_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FB_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FB_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FB_INTR` writer"]
pub struct W(crate::W<FB_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FB_INTR_SPEC>;
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
impl From<crate::W<FB_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FB_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FB_0_INTR` reader - Active high edge interrupt detected for Fabric. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FB_0_INTR_R(crate::FieldReader<bool, bool>);
impl FB_0_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_0_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_0_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_0_INTR` writer - Active high edge interrupt detected for Fabric. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FB_0_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_0_INTR_W<'a> {
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
#[doc = "Field `FB_1_INTR` reader - Active high edge interrupt detected for Fabric. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FB_1_INTR_R(crate::FieldReader<bool, bool>);
impl FB_1_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_1_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_1_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_1_INTR` writer - Active high edge interrupt detected for Fabric. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FB_1_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_1_INTR_W<'a> {
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
#[doc = "Field `FB_2_INTR` reader - Active high edge interrupt detected for Fabric. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FB_2_INTR_R(crate::FieldReader<bool, bool>);
impl FB_2_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_2_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_2_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_2_INTR` writer - Active high edge interrupt detected for Fabric. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FB_2_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_2_INTR_W<'a> {
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
#[doc = "Field `FB_3_INTR` reader - Active high edge interrupt detected for Fabric. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FB_3_INTR_R(crate::FieldReader<bool, bool>);
impl FB_3_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_3_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_3_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_3_INTR` writer - Active high edge interrupt detected for Fabric. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FB_3_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_3_INTR_W<'a> {
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
    #[doc = "Bit 0 - Active high edge interrupt detected for Fabric. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn fb_0_intr(&self) -> FB_0_INTR_R {
        FB_0_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Active high edge interrupt detected for Fabric. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn fb_1_intr(&self) -> FB_1_INTR_R {
        FB_1_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Active high edge interrupt detected for Fabric. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn fb_2_intr(&self) -> FB_2_INTR_R {
        FB_2_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Active high edge interrupt detected for Fabric. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn fb_3_intr(&self) -> FB_3_INTR_R {
        FB_3_INTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active high edge interrupt detected for Fabric. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn fb_0_intr(&mut self) -> FB_0_INTR_W {
        FB_0_INTR_W { w: self }
    }
    #[doc = "Bit 1 - Active high edge interrupt detected for Fabric. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn fb_1_intr(&mut self) -> FB_1_INTR_W {
        FB_1_INTR_W { w: self }
    }
    #[doc = "Bit 2 - Active high edge interrupt detected for Fabric. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn fb_2_intr(&mut self) -> FB_2_INTR_W {
        FB_2_INTR_W { w: self }
    }
    #[doc = "Bit 3 - Active high edge interrupt detected for Fabric. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn fb_3_intr(&mut self) -> FB_3_INTR_W {
        FB_3_INTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indicators of interrupt triggers detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fb_intr](index.html) module"]
pub struct FB_INTR_SPEC;
impl crate::RegisterSpec for FB_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fb_intr::R](R) reader structure"]
impl crate::Readable for FB_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fb_intr::W](W) writer structure"]
impl crate::Writable for FB_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FB_INTR to value 0"]
impl crate::Resettable for FB_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
