#[doc = "Register `RDATA` reader"]
pub struct R(crate::R<RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDATA` writer"]
pub struct W(crate::W<RDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDATA_SPEC>;
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
impl From<crate::W<RDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDATA` reader - Read data from I2C to Wishbone master is registered"]
pub struct RDATA_R(crate::FieldReader<u8, u8>);
impl RDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDATA` writer - Read data from I2C to Wishbone master is registered"]
pub struct RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> RDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Read data from I2C to Wishbone master is registered"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read data from I2C to Wishbone master is registered"]
    #[inline(always)]
    pub fn rdata(&mut self) -> RDATA_W {
        RDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read data from I2C to Wishbone master is registered\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdata](index.html) module"]
pub struct RDATA_SPEC;
impl crate::RegisterSpec for RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdata::R](R) reader structure"]
impl crate::Readable for RDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rdata::W](W) writer structure"]
impl crate::Writable for RDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RDATA to value 0"]
impl crate::Resettable for RDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
