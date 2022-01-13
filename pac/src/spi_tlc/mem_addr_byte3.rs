#[doc = "Register `MemAddrByte3` reader"]
pub struct R(crate::R<MEMADDRBYTE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMADDRBYTE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMADDRBYTE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMADDRBYTE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MemAddrByte3` writer"]
pub struct W(crate::W<MEMADDRBYTE3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMADDRBYTE3_SPEC>;
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
impl From<crate::W<MEMADDRBYTE3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMADDRBYTE3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MemAddrByte3` reader - AHB Memory Address, It is representing AHB Byte Address bit \\[31:24\\]. Bit 7:0 R/W,Default All 0"]
pub struct MEMADDRBYTE3_R(crate::FieldReader<u8, u8>);
impl MEMADDRBYTE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEMADDRBYTE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMADDRBYTE3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MemAddrByte3` writer - AHB Memory Address, It is representing AHB Byte Address bit \\[31:24\\]. Bit 7:0 R/W,Default All 0"]
pub struct MEMADDRBYTE3_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMADDRBYTE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - AHB Memory Address, It is representing AHB Byte Address bit \\[31:24\\]. Bit 7:0 R/W,Default All 0"]
    #[inline(always)]
    pub fn mem_addr_byte3(&self) -> MEMADDRBYTE3_R {
        MEMADDRBYTE3_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AHB Memory Address, It is representing AHB Byte Address bit \\[31:24\\]. Bit 7:0 R/W,Default All 0"]
    #[inline(always)]
    pub fn mem_addr_byte3(&mut self) -> MEMADDRBYTE3_W {
        MEMADDRBYTE3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Memory Address, It is representing AHB Byte Address bit \\[31:24\\]. Bit 7:0 R/W,Default All 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_addr_byte3](index.html) module"]
pub struct MEMADDRBYTE3_SPEC;
impl crate::RegisterSpec for MEMADDRBYTE3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mem_addr_byte3::R](R) reader structure"]
impl crate::Readable for MEMADDRBYTE3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_addr_byte3::W](W) writer structure"]
impl crate::Writable for MEMADDRBYTE3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MemAddrByte3 to value 0"]
impl crate::Resettable for MEMADDRBYTE3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
