#[doc = "Register `RAMFIFO1` reader"]
pub struct R(crate::R<RAMFIFO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMFIFO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMFIFO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMFIFO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMFIFO1` writer"]
pub struct W(crate::W<RAMFIFO1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMFIFO1_SPEC>;
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
impl From<crate::W<RAMFIFO1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMFIFO1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMFIFO1` reader - RAMFIFO1 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO1. From 0x9000 to 0x9FFC."]
pub struct RAMFIFO1_R(crate::FieldReader<u32, u32>);
impl RAMFIFO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RAMFIFO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMFIFO1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMFIFO1` writer - RAMFIFO1 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO1. From 0x9000 to 0x9FFC."]
pub struct RAMFIFO1_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMFIFO1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - RAMFIFO1 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO1. From 0x9000 to 0x9FFC."]
    #[inline(always)]
    pub fn ramfifo1(&self) -> RAMFIFO1_R {
        RAMFIFO1_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - RAMFIFO1 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO1. From 0x9000 to 0x9FFC."]
    #[inline(always)]
    pub fn ramfifo1(&mut self) -> RAMFIFO1_W {
        RAMFIFO1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMFIFO1 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO1. From 0x9000 to 0x9FFC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramfifo1](index.html) module"]
pub struct RAMFIFO1_SPEC;
impl crate::RegisterSpec for RAMFIFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramfifo1::R](R) reader structure"]
impl crate::Readable for RAMFIFO1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramfifo1::W](W) writer structure"]
impl crate::Writable for RAMFIFO1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAMFIFO1 to value 0"]
impl crate::Resettable for RAMFIFO1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
