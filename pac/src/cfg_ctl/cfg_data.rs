#[doc = "Register `CFG_DATA` reader"]
pub struct R(crate::R<CFG_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_DATA` writer"]
pub struct W(crate::W<CFG_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_DATA_SPEC>;
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
impl From<crate::W<CFG_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG_DATA` reader - Configuration Data: ARM firmware/software Access this register to Read/Write the configuration bit cells."]
pub struct CFG_DATA_R(crate::FieldReader<u32, u32>);
impl CFG_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CFG_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG_DATA` writer - Configuration Data: ARM firmware/software Access this register to Read/Write the configuration bit cells."]
pub struct CFG_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Configuration Data: ARM firmware/software Access this register to Read/Write the configuration bit cells."]
    #[inline(always)]
    pub fn cfg_data(&self) -> CFG_DATA_R {
        CFG_DATA_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configuration Data: ARM firmware/software Access this register to Read/Write the configuration bit cells."]
    #[inline(always)]
    pub fn cfg_data(&mut self) -> CFG_DATA_W {
        CFG_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Data: ARM firmware/software Access this register to Read/Write the configuration bit cells.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_data](index.html) module"]
pub struct CFG_DATA_SPEC;
impl crate::RegisterSpec for CFG_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_data::R](R) reader structure"]
impl crate::Readable for CFG_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_data::W](W) writer structure"]
impl crate::Writable for CFG_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_DATA to value 0"]
impl crate::Resettable for CFG_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
