#[doc = "Register `CFG_CTRL` reader"]
pub struct R(crate::R<CFG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_CTRL` writer"]
pub struct W(crate::W<CFG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_CTRL_SPEC>;
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
impl From<crate::W<CFG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG_IN_SEL` reader - Selects source APB master to SPI Master between M4/AP and Fabric"]
pub struct CFG_IN_SEL_R(crate::FieldReader<bool, bool>);
impl CFG_IN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFG_IN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_IN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG_IN_SEL` writer - Selects source APB master to SPI Master between M4/AP and Fabric"]
pub struct CFG_IN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_IN_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Selects source APB master to SPI Master between M4/AP and Fabric"]
    #[inline(always)]
    pub fn cfg_in_sel(&self) -> CFG_IN_SEL_R {
        CFG_IN_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects source APB master to SPI Master between M4/AP and Fabric"]
    #[inline(always)]
    pub fn cfg_in_sel(&mut self) -> CFG_IN_SEL_W {
        CFG_IN_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_ctrl](index.html) module"]
pub struct CFG_CTRL_SPEC;
impl crate::RegisterSpec for CFG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_ctrl::R](R) reader structure"]
impl crate::Readable for CFG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_ctrl::W](W) writer structure"]
impl crate::Writable for CFG_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_CTRL to value 0"]
impl crate::Resettable for CFG_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
