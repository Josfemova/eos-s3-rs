#[doc = "Register `AHBACCESSCTL` reader"]
pub struct R(crate::R<AHBACCESSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBACCESSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBACCESSCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBACCESSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBACCESSCTL` writer"]
pub struct W(crate::W<AHBACCESSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBACCESSCTL_SPEC>;
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
impl From<crate::W<AHBACCESSCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBACCESSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AhbReadReqMode` reader - AHB Memory Read Option \n 2'b11 : If MemAddrByte\\[1:0\\]
is 2'b11, then AHB Memory Read will not be automatically triggered once MemAddrByte1 is written Other : The AHB Memory Read behavior is same as S2, Once MemAddrByte1 is written, AHB Memory read is triggered."]
pub struct AHBREADREQMODE_R(crate::FieldReader<u8, u8>);
impl AHBREADREQMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AHBREADREQMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBREADREQMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AhbReadReqMode` writer - AHB Memory Read Option \n 2'b11 : If MemAddrByte\\[1:0\\]
is 2'b11, then AHB Memory Read will not be automatically triggered once MemAddrByte1 is written Other : The AHB Memory Read behavior is same as S2, Once MemAddrByte1 is written, AHB Memory read is triggered."]
pub struct AHBREADREQMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBREADREQMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "Field `scratch0` reader - General Purpose Registers"]
pub struct SCRATCH0_R(crate::FieldReader<u8, u8>);
impl SCRATCH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCRATCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRATCH0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `scratch0` writer - General Purpose Registers"]
pub struct SCRATCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x3f << 2)) | ((value as u8 & 0x3f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - AHB Memory Read Option \n 2'b11 : If MemAddrByte\\[1:0\\]
is 2'b11, then AHB Memory Read will not be automatically triggered once MemAddrByte1 is written Other : The AHB Memory Read behavior is same as S2, Once MemAddrByte1 is written, AHB Memory read is triggered."]
    #[inline(always)]
    pub fn ahb_read_req_mode(&self) -> AHBREADREQMODE_R {
        AHBREADREQMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:7 - General Purpose Registers"]
    #[inline(always)]
    pub fn scratch0(&self) -> SCRATCH0_R {
        SCRATCH0_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AHB Memory Read Option \n 2'b11 : If MemAddrByte\\[1:0\\]
is 2'b11, then AHB Memory Read will not be automatically triggered once MemAddrByte1 is written Other : The AHB Memory Read behavior is same as S2, Once MemAddrByte1 is written, AHB Memory read is triggered."]
    #[inline(always)]
    pub fn ahb_read_req_mode(&mut self) -> AHBREADREQMODE_W {
        AHBREADREQMODE_W { w: self }
    }
    #[doc = "Bits 2:7 - General Purpose Registers"]
    #[inline(always)]
    pub fn scratch0(&mut self) -> SCRATCH0_W {
        SCRATCH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbaccessctl](index.html) module"]
pub struct AHBACCESSCTL_SPEC;
impl crate::RegisterSpec for AHBACCESSCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ahbaccessctl::R](R) reader structure"]
impl crate::Readable for AHBACCESSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbaccessctl::W](W) writer structure"]
impl crate::Writable for AHBACCESSCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBACCESSCTL to value 0"]
impl crate::Resettable for AHBACCESSCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
