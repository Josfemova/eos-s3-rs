#[doc = "Register `RAMFIFO2` reader"]
pub struct R(crate::R<RAMFIFO2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMFIFO2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMFIFO2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMFIFO2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMFIFO2` writer"]
pub struct W(crate::W<RAMFIFO2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMFIFO2_SPEC>;
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
impl From<crate::W<RAMFIFO2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMFIFO2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMFIFO2` reader - RAMFIFO2 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO2. From 0xA000 to 0xAFFC."]
pub struct RAMFIFO2_R(crate::FieldReader<u32, u32>);
impl RAMFIFO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RAMFIFO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMFIFO2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMFIFO2` writer - RAMFIFO2 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO2. From 0xA000 to 0xAFFC."]
pub struct RAMFIFO2_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMFIFO2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - RAMFIFO2 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO2. From 0xA000 to 0xAFFC."]
    #[inline(always)]
    pub fn ramfifo2(&self) -> RAMFIFO2_R {
        RAMFIFO2_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - RAMFIFO2 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO2. From 0xA000 to 0xAFFC."]
    #[inline(always)]
    pub fn ramfifo2(&mut self) -> RAMFIFO2_W {
        RAMFIFO2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMFIFO2 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO2. From 0xA000 to 0xAFFC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramfifo2](index.html) module"]
pub struct RAMFIFO2_SPEC;
impl crate::RegisterSpec for RAMFIFO2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramfifo2::R](R) reader structure"]
impl crate::Readable for RAMFIFO2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramfifo2::W](W) writer structure"]
impl crate::Writable for RAMFIFO2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAMFIFO2 to value 0"]
impl crate::Resettable for RAMFIFO2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
