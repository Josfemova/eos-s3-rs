#[doc = "Register `MemAddrByte2` reader"]
pub struct R(crate::R<MEMADDRBYTE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMADDRBYTE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMADDRBYTE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMADDRBYTE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MemAddrByte2` writer"]
pub struct W(crate::W<MEMADDRBYTE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMADDRBYTE2_SPEC>;
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
impl From<crate::W<MEMADDRBYTE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMADDRBYTE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MemAddrByte2` reader - AHB Memory Address, It is representing AHB Byte Address bit \\[23:16\\]. Bit 7:0 R/W, Default All 0"]
pub struct MEMADDRBYTE2_R(crate::FieldReader<u8, u8>);
impl MEMADDRBYTE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEMADDRBYTE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMADDRBYTE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MemAddrByte2` writer - AHB Memory Address, It is representing AHB Byte Address bit \\[23:16\\]. Bit 7:0 R/W, Default All 0"]
pub struct MEMADDRBYTE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMADDRBYTE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - AHB Memory Address, It is representing AHB Byte Address bit \\[23:16\\]. Bit 7:0 R/W, Default All 0"]
    #[inline(always)]
    pub fn mem_addr_byte2(&self) -> MEMADDRBYTE2_R {
        MEMADDRBYTE2_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AHB Memory Address, It is representing AHB Byte Address bit \\[23:16\\]. Bit 7:0 R/W, Default All 0"]
    #[inline(always)]
    pub fn mem_addr_byte2(&mut self) -> MEMADDRBYTE2_W {
        MEMADDRBYTE2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Memory Address, It is representing AHB Byte Address bit \\[23:16\\]. Bit 7:0 R/W, Default All 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_addr_byte2](index.html) module"]
pub struct MEMADDRBYTE2_SPEC;
impl crate::RegisterSpec for MEMADDRBYTE2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mem_addr_byte2::R](R) reader structure"]
impl crate::Readable for MEMADDRBYTE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_addr_byte2::W](W) writer structure"]
impl crate::Writable for MEMADDRBYTE2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MemAddrByte2 to value 0"]
impl crate::Resettable for MEMADDRBYTE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
