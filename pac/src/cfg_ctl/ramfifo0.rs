#[doc = "Register `RAMFIFO0` reader"]
pub struct R(crate::R<RAMFIFO0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMFIFO0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMFIFO0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMFIFO0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMFIFO0` writer"]
pub struct W(crate::W<RAMFIFO0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMFIFO0_SPEC>;
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
impl From<crate::W<RAMFIFO0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMFIFO0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMFIFO0` reader - RAMFIFO0 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO0. From 0x8000 to 0x8FFC."]
pub struct RAMFIFO0_R(crate::FieldReader<u32, u32>);
impl RAMFIFO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RAMFIFO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMFIFO0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMFIFO0` writer - RAMFIFO0 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO0. From 0x8000 to 0x8FFC."]
pub struct RAMFIFO0_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMFIFO0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - RAMFIFO0 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO0. From 0x8000 to 0x8FFC."]
    #[inline(always)]
    pub fn ramfifo0(&self) -> RAMFIFO0_R {
        RAMFIFO0_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - RAMFIFO0 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO0. From 0x8000 to 0x8FFC."]
    #[inline(always)]
    pub fn ramfifo0(&mut self) -> RAMFIFO0_W {
        RAMFIFO0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMFIFO0 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO0. From 0x8000 to 0x8FFC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramfifo0](index.html) module"]
pub struct RAMFIFO0_SPEC;
impl crate::RegisterSpec for RAMFIFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramfifo0::R](R) reader structure"]
impl crate::Readable for RAMFIFO0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramfifo0::W](W) writer structure"]
impl crate::Writable for RAMFIFO0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAMFIFO0 to value 0"]
impl crate::Resettable for RAMFIFO0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
