#[doc = "Register `MemDataByte2` reader"]
pub struct R(crate::R<MEMDATABYTE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMDATABYTE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMDATABYTE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMDATABYTE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MemDataByte2` writer"]
pub struct W(crate::W<MEMDATABYTE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMDATABYTE2_SPEC>;
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
impl From<crate::W<MEMDATABYTE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMDATABYTE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MemDataByte2` reader - Third byte of AHB memory data"]
pub struct MEMDATABYTE2_R(crate::FieldReader<u8, u8>);
impl MEMDATABYTE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEMDATABYTE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMDATABYTE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MemDataByte2` writer - Third byte of AHB memory data"]
pub struct MEMDATABYTE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMDATABYTE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Third byte of AHB memory data"]
    #[inline(always)]
    pub fn mem_data_byte2(&self) -> MEMDATABYTE2_R {
        MEMDATABYTE2_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Third byte of AHB memory data"]
    #[inline(always)]
    pub fn mem_data_byte2(&mut self) -> MEMDATABYTE2_W {
        MEMDATABYTE2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Third byte of AHB memory data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_data_byte2](index.html) module"]
pub struct MEMDATABYTE2_SPEC;
impl crate::RegisterSpec for MEMDATABYTE2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mem_data_byte2::R](R) reader structure"]
impl crate::Readable for MEMDATABYTE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_data_byte2::W](W) writer structure"]
impl crate::Writable for MEMDATABYTE2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MemDataByte2 to value 0"]
impl crate::Resettable for MEMDATABYTE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
