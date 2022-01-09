#[doc = "Register `SM1_RUNTIME_ADDR_CTRL` reader"]
pub struct R(crate::R<SM1_RUNTIME_ADDR_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM1_RUNTIME_ADDR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SM1_RUNTIME_ADDR_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SM1_RUNTIME_ADDR_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SM1_RUNTIME_ADDR_CTRL` writer"]
pub struct W(crate::W<SM1_RUNTIME_ADDR_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SM1_RUNTIME_ADDR_CTRL_SPEC>;
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
impl From<crate::W<SM1_RUNTIME_ADDR_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SM1_RUNTIME_ADDR_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SM1_RUNTIME_ADDR_CTRL` reader - Write a '1' to Toggle signal used to signal when a new value has been written"]
pub struct SM1_RUNTIME_ADDR_CTRL_R(crate::FieldReader<bool, bool>);
impl SM1_RUNTIME_ADDR_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM1_RUNTIME_ADDR_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM1_RUNTIME_ADDR_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM1_RUNTIME_ADDR_CTRL` writer - Write a '1' to Toggle signal used to signal when a new value has been written"]
pub struct SM1_RUNTIME_ADDR_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_RUNTIME_ADDR_CTRL_W<'a> {
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
    #[doc = "Bit 0 - Write a '1' to Toggle signal used to signal when a new value has been written"]
    #[inline(always)]
    pub fn sm1_runtime_addr_ctrl(&self) -> SM1_RUNTIME_ADDR_CTRL_R {
        SM1_RUNTIME_ADDR_CTRL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a '1' to Toggle signal used to signal when a new value has been written"]
    #[inline(always)]
    pub fn sm1_runtime_addr_ctrl(&mut self) -> SM1_RUNTIME_ADDR_CTRL_W {
        SM1_RUNTIME_ADDR_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Used to toggle signal used to signal when a new value has been written.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sm1_runtime_addr_ctrl](index.html) module"]
pub struct SM1_RUNTIME_ADDR_CTRL_SPEC;
impl crate::RegisterSpec for SM1_RUNTIME_ADDR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sm1_runtime_addr_ctrl::R](R) reader structure"]
impl crate::Readable for SM1_RUNTIME_ADDR_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sm1_runtime_addr_ctrl::W](W) writer structure"]
impl crate::Writable for SM1_RUNTIME_ADDR_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SM1_RUNTIME_ADDR_CTRL to value 0"]
impl crate::Resettable for SM1_RUNTIME_ADDR_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
