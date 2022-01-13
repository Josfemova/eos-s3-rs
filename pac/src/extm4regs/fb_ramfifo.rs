#[doc = "Register `FB_RAMFIFO` reader"]
pub struct R(crate::R<FB_RAMFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FB_RAMFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FB_RAMFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FB_RAMFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FB_RAMFIFO` writer"]
pub struct W(crate::W<FB_RAMFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FB_RAMFIFO_SPEC>;
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
impl From<crate::W<FB_RAMFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FB_RAMFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FB_RAMFIFO_MODE` reader - Sets Fabric in APB mode"]
pub struct FB_RAMFIFO_MODE_R(crate::FieldReader<bool, bool>);
impl FB_RAMFIFO_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_RAMFIFO_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_RAMFIFO_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_RAMFIFO_MODE` writer - Sets Fabric in APB mode"]
pub struct FB_RAMFIFO_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_RAMFIFO_MODE_W<'a> {
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
    #[doc = "Bit 0 - Sets Fabric in APB mode"]
    #[inline(always)]
    pub fn fb_ramfifo_mode(&self) -> FB_RAMFIFO_MODE_R {
        FB_RAMFIFO_MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sets Fabric in APB mode"]
    #[inline(always)]
    pub fn fb_ramfifo_mode(&mut self) -> FB_RAMFIFO_MODE_W {
        FB_RAMFIFO_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sets Fabric in APB mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fb_ramfifo](index.html) module"]
pub struct FB_RAMFIFO_SPEC;
impl crate::RegisterSpec for FB_RAMFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fb_ramfifo::R](R) reader structure"]
impl crate::Readable for FB_RAMFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fb_ramfifo::W](W) writer structure"]
impl crate::Writable for FB_RAMFIFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FB_RAMFIFO to value 0"]
impl crate::Resettable for FB_RAMFIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
