#[doc = "Register `MAX_BL_CNT` reader"]
pub struct R(crate::R<MAX_BL_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAX_BL_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAX_BL_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAX_BL_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAX_BL_CNT` writer"]
pub struct W(crate::W<MAX_BL_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAX_BL_CNT_SPEC>;
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
impl From<crate::W<MAX_BL_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAX_BL_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAX_BL_CNT` reader - Maximum Bit Length Count: ARM firmware/software sets this register 1'b1 to set the Maximum Bit Line Count"]
pub struct MAX_BL_CNT_R(crate::FieldReader<u32, u32>);
impl MAX_BL_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MAX_BL_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAX_BL_CNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAX_BL_CNT` writer - Maximum Bit Length Count: ARM firmware/software sets this register 1'b1 to set the Maximum Bit Line Count"]
pub struct MAX_BL_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_BL_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Maximum Bit Length Count: ARM firmware/software sets this register 1'b1 to set the Maximum Bit Line Count"]
    #[inline(always)]
    pub fn max_bl_cnt(&self) -> MAX_BL_CNT_R {
        MAX_BL_CNT_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Maximum Bit Length Count: ARM firmware/software sets this register 1'b1 to set the Maximum Bit Line Count"]
    #[inline(always)]
    pub fn max_bl_cnt(&mut self) -> MAX_BL_CNT_W {
        MAX_BL_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Maximum Bit Length Count: ARM firmware/software sets this register 1'b1 to set the Maximum Bit Line Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [max_bl_cnt](index.html) module"]
pub struct MAX_BL_CNT_SPEC;
impl crate::RegisterSpec for MAX_BL_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [max_bl_cnt::R](R) reader structure"]
impl crate::Readable for MAX_BL_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [max_bl_cnt::W](W) writer structure"]
impl crate::Writable for MAX_BL_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAX_BL_CNT to value 0x2d"]
impl crate::Resettable for MAX_BL_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2d
    }
}
