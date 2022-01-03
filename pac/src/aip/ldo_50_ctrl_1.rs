#[doc = "Register `LDO_50_CTRL_1` reader"]
pub struct R(crate::R<LDO_50_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO_50_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO_50_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO_50_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDO_50_CTRL_1` writer"]
pub struct W(crate::W<LDO_50_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO_50_CTRL_1_SPEC>;
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
impl From<crate::W<LDO_50_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO_50_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEST` reader - Lab test and internal block characterization test control pins."]
pub struct TEST_R(crate::FieldReader<u8, u8>);
impl TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST` writer - Lab test and internal block characterization test control pins."]
pub struct TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TESTREQ` reader - Request of connection of the anatestbus to an external pin for characterization"]
pub struct TESTREQ_R(crate::FieldReader<bool, bool>);
impl TESTREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TESTREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TESTREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Lab test and internal block characterization test control pins."]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Request of connection of the anatestbus to an external pin for characterization"]
    #[inline(always)]
    pub fn testreq(&self) -> TESTREQ_R {
        TESTREQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Lab test and internal block characterization test control pins."]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W {
        TEST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LDO_50 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo_50_ctrl_1](index.html) module"]
pub struct LDO_50_CTRL_1_SPEC;
impl crate::RegisterSpec for LDO_50_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo_50_ctrl_1::R](R) reader structure"]
impl crate::Readable for LDO_50_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo_50_ctrl_1::W](W) writer structure"]
impl crate::Writable for LDO_50_CTRL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LDO_50_CTRL_1 to value 0"]
impl crate::Resettable for LDO_50_CTRL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
