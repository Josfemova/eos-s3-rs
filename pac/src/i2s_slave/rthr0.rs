#[doc = "Register `RTHR0` writer"]
pub struct W(crate::W<RTHR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTHR0_SPEC>;
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
impl From<crate::W<RTHR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTHR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTHR0` writer - The right stereo data to be transmitted serially through the transmit channel output (sdox) is written through this register. Writing is a two-stage process: (1) A left stereo sample MUST first be written to the LTHRx register. (2) A write to this register passes the right stereo sample to the transmitter. Data should only be written to the FIFO when it is not full. Any attempt to write to a full FIFO results in that data being lost and an overrun interrupt being generated."]
pub struct RTHR0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTHR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - The right stereo data to be transmitted serially through the transmit channel output (sdox) is written through this register. Writing is a two-stage process: (1) A left stereo sample MUST first be written to the LTHRx register. (2) A write to this register passes the right stereo sample to the transmitter. Data should only be written to the FIFO when it is not full. Any attempt to write to a full FIFO results in that data being lost and an overrun interrupt being generated."]
    #[inline(always)]
    pub fn rthr0(&mut self) -> RTHR0_W {
        RTHR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Right Transmit Holding Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rthr0](index.html) module"]
pub struct RTHR0_SPEC;
impl crate::RegisterSpec for RTHR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rthr0::W](W) writer structure"]
impl crate::Writable for RTHR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTHR0 to value 0"]
impl crate::Resettable for RTHR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
