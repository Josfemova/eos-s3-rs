#[doc = "Register `C31_CLK_DIV` reader"]
pub struct R(crate::R<C31_CLK_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C31_CLK_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C31_CLK_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C31_CLK_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C31_CLK_DIV` writer"]
pub struct W(crate::W<C31_CLK_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C31_CLK_DIV_SPEC>;
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
impl From<crate::W<C31_CLK_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C31_CLK_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C01_CLK_DIV` reader - The input clock frequency will be divided and generate the corresponding clock output. div is reg value + 1"]
pub struct C01_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl C01_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        C01_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C01_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C01_CLK_DIV` writer - The input clock frequency will be divided and generate the corresponding clock output. div is reg value + 1"]
pub struct C01_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> C01_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "This bit is used to turn off the clock for the SYNC down Divider\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C01_CLK_DIV_CG_A {
    #[doc = "0: Clock is stop"]
    STOP = 0,
    #[doc = "1: Clock is runnig"]
    RUN = 1,
}
impl From<C01_CLK_DIV_CG_A> for bool {
    #[inline(always)]
    fn from(variant: C01_CLK_DIV_CG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C01_CLK_DIV_CG` reader - This bit is used to turn off the clock for the SYNC down Divider"]
pub struct C01_CLK_DIV_CG_R(crate::FieldReader<bool, C01_CLK_DIV_CG_A>);
impl C01_CLK_DIV_CG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        C01_CLK_DIV_CG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C01_CLK_DIV_CG_A {
        match self.bits {
            false => C01_CLK_DIV_CG_A::STOP,
            true => C01_CLK_DIV_CG_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == C01_CLK_DIV_CG_A::STOP
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == C01_CLK_DIV_CG_A::RUN
    }
}
impl core::ops::Deref for C01_CLK_DIV_CG_R {
    type Target = crate::FieldReader<bool, C01_CLK_DIV_CG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C01_CLK_DIV_CG` writer - This bit is used to turn off the clock for the SYNC down Divider"]
pub struct C01_CLK_DIV_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> C01_CLK_DIV_CG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C01_CLK_DIV_CG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(C01_CLK_DIV_CG_A::STOP)
    }
    #[doc = "Clock is runnig"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(C01_CLK_DIV_CG_A::RUN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - The input clock frequency will be divided and generate the corresponding clock output. div is reg value + 1"]
    #[inline(always)]
    pub fn c01_clk_div(&self) -> C01_CLK_DIV_R {
        C01_CLK_DIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - This bit is used to turn off the clock for the SYNC down Divider"]
    #[inline(always)]
    pub fn c01_clk_div_cg(&self) -> C01_CLK_DIV_CG_R {
        C01_CLK_DIV_CG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - The input clock frequency will be divided and generate the corresponding clock output. div is reg value + 1"]
    #[inline(always)]
    pub fn c01_clk_div(&mut self) -> C01_CLK_DIV_W {
        C01_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 4 - This bit is used to turn off the clock for the SYNC down Divider"]
    #[inline(always)]
    pub fn c01_clk_div_cg(&mut self) -> C01_CLK_DIV_CG_W {
        C01_CLK_DIV_CG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source Clock is C30 (LPSD CLK).If Bit 4 is 0, any change on Bit 3:0 will not take effect. And bit 4 and bit 3:0 can not change at same time.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c31_clk_div](index.html) module"]
pub struct C31_CLK_DIV_SPEC;
impl crate::RegisterSpec for C31_CLK_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c31_clk_div::R](R) reader structure"]
impl crate::Readable for C31_CLK_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c31_clk_div::W](W) writer structure"]
impl crate::Writable for C31_CLK_DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C31_CLK_DIV to value 0x13"]
impl crate::Resettable for C31_CLK_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x13
    }
}
