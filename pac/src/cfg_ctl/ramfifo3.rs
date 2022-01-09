#[doc = "Register `RAMFIFO3` reader"]
pub struct R(crate::R<RAMFIFO3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMFIFO3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMFIFO3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMFIFO3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMFIFO3` writer"]
pub struct W(crate::W<RAMFIFO3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMFIFO3_SPEC>;
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
impl From<crate::W<RAMFIFO3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMFIFO3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMFIFO3` reader - RAMFIFO3 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO3. From 0xB000 to 0xBFFC."]
pub struct RAMFIFO3_R(crate::FieldReader<u32, u32>);
impl RAMFIFO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RAMFIFO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMFIFO3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMFIFO3` writer - RAMFIFO3 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO3. From 0xB000 to 0xBFFC."]
pub struct RAMFIFO3_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMFIFO3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - RAMFIFO3 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO3. From 0xB000 to 0xBFFC."]
    #[inline(always)]
    pub fn ramfifo3(&self) -> RAMFIFO3_R {
        RAMFIFO3_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - RAMFIFO3 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO3. From 0xB000 to 0xBFFC."]
    #[inline(always)]
    pub fn ramfifo3(&mut self) -> RAMFIFO3_W {
        RAMFIFO3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMFIFO3 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO3. From 0xB000 to 0xBFFC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramfifo3](index.html) module"]
pub struct RAMFIFO3_SPEC;
impl crate::RegisterSpec for RAMFIFO3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramfifo3::R](R) reader structure"]
impl crate::Readable for RAMFIFO3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramfifo3::W](W) writer structure"]
impl crate::Writable for RAMFIFO3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAMFIFO3 to value 0"]
impl crate::Resettable for RAMFIFO3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
