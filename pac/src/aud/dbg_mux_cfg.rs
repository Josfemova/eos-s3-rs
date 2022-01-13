#[doc = "Register `DBG_MUX_CFG` reader"]
pub struct R(crate::R<DBG_MUX_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_MUX_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_MUX_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_MUX_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_MUX_CFG` writer"]
pub struct W(crate::W<DBG_MUX_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_MUX_CFG_SPEC>;
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
impl From<crate::W<DBG_MUX_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_MUX_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_MUX` reader - \n 0000 => dbg_tp\\[7:0\\]
= dbg_fifo_0a_wptr\\[7:0\\]
\n 0001 => dbg_tp\\[7:0\\]
= dbg_fifo_0a_rptr\\[7:0\\]
\n 0010 => dbg_tp\\[7:0\\]
= {1'b0, dbg_fifo_0a_rptr\\[10:8\\], 1'b0, dbg_fifo_0a_wptr\\[10:8\\]} \n\n 0011 => dbg_tp\\[7:0\\]
= dbg_fifo_1a_wptr\\[7:0\\]
\n 0100 => dbg_tp\\[7:0\\]
= dbg_fifo_1a_rptr\\[7:0\\]
\n 0101 => dbg_tp\\[7:0\\]
= {1'b0, dbg_fifo_1a_rptr\\[10:8\\], 1'b0, dbg_fifo_1a_wptr\\[10:8\\]} \n\n 0110 => dbg_tp\\[7:0\\]
= {2'd0, dbg_fifo_0b_wptr\\[5:0\\]} \n\n 0111 => dbg_tp\\[7:0\\]
= {2'd0, dbg_fifo_0b_rptr\\[5:0\\]} \n\n 1000 => dbg_tp\\[7:0\\]
= {2'd0, dbg_fifo_1b_wptr\\[5:0\\]} \n\n 1001 => dbg_tp\\[7:0\\]
= {2'd0, dbg_fifo_1b_rptr\\[5:0\\]} \n\n 1010 => dbg_tp\\[7:0\\]
= {2'd0, dbg_afifo_0_rptr\\[1:0\\], 2'd0, dbg_afifo_0_wptr\\[1:0\\]} \n\n 1011 => dbg_tp\\[7:0\\]
={2'd0, dbg_afifo_1_rptr\\[1:0\\], 2'd0, dbg_afifo_1_wptr\\[1:0\\]} \n\n 1100 => dbg_tp\\[7:0\\]
= {PCM_DATA_L\\[7:1\\],VALID} \n\n 1101 => dbg_tp\\[7:0\\]
= {PCM_DATA_L\\[15:9\\],VALID} \n\n 1110 => dbg_tp\\[7:0\\]
= {PCM_DATA_R\\[7:1\\],VALID} \n\n 1111 => dbg_tp\\[7:0\\]
= {PCM_DATA_R\\[15:9\\],VALID}"]
pub struct DBG_MUX_R(crate::FieldReader<u8, u8>);
impl DBG_MUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBG_MUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_MUX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_MUX` writer - \n 0000 => dbg_tp\\[7:0\\]
= dbg_fifo_0a_wptr\\[7:0\\]
\n 0001 => dbg_tp\\[7:0\\]
= dbg_fifo_0a_rptr\\[7:0\\]
\n 0010 => dbg_tp\\[7:0\\]
= {1'b0, dbg_fifo_0a_rptr\\[10:8\\], 1'b0, dbg_fifo_0a_wptr\\[10:8\\]} \n\n 0011 => dbg_tp\\[7:0\\]
= dbg_fifo_1a_wptr\\[7:0\\]
\n 0100 => dbg_tp\\[7:0\\]
= dbg_fifo_1a_rptr\\[7:0\\]
\n 0101 => dbg_tp\\[7:0\\]
= {1'b0, dbg_fifo_1a_rptr\\[10:8\\], 1'b0, dbg_fifo_1a_wptr\\[10:8\\]} \n\n 0110 => dbg_tp\\[7:0\\]
= {2'd0, dbg_fifo_0b_wptr\\[5:0\\]} \n\n 0111 => dbg_tp\\[7:0\\]
= {2'd0, dbg_fifo_0b_rptr\\[5:0\\]} \n\n 1000 => dbg_tp\\[7:0\\]
= {2'd0, dbg_fifo_1b_wptr\\[5:0\\]} \n\n 1001 => dbg_tp\\[7:0\\]
= {2'd0, dbg_fifo_1b_rptr\\[5:0\\]} \n\n 1010 => dbg_tp\\[7:0\\]
= {2'd0, dbg_afifo_0_rptr\\[1:0\\], 2'd0, dbg_afifo_0_wptr\\[1:0\\]} \n\n 1011 => dbg_tp\\[7:0\\]
={2'd0, dbg_afifo_1_rptr\\[1:0\\], 2'd0, dbg_afifo_1_wptr\\[1:0\\]} \n\n 1100 => dbg_tp\\[7:0\\]
= {PCM_DATA_L\\[7:1\\],VALID} \n\n 1101 => dbg_tp\\[7:0\\]
= {PCM_DATA_L\\[15:9\\],VALID} \n\n 1110 => dbg_tp\\[7:0\\]
= {PCM_DATA_R\\[7:1\\],VALID} \n\n 1111 => dbg_tp\\[7:0\\]
= {PCM_DATA_R\\[15:9\\],VALID}"]
pub struct DBG_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_MUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - \n 0000 => dbg_tp\\[7:0\\]
= dbg_fifo_0a_wptr\\[7:0\\]
\n 0001 => dbg_tp\\[7:0\\]
= dbg_fifo_0a_rptr\\[7:0\\]
\n 0010 => dbg_tp\\[7:0\\]
= {1'b0, dbg_fifo_0a_rptr\\[10:8\\], 1'b0, dbg_fifo_0a_wptr\\[10:8\\]} \n\n 0011 => dbg_tp\\[7:0\\]
= dbg_fifo_1a_wptr\\[7:0\\]
\n 0100 => dbg_tp\\[7:0\\]
= dbg_fifo_1a_rptr\\[7:0\\]
\n 0101 => dbg_tp\\[7:0\\]
= {1'b0, dbg_fifo_1a_rptr\\[10:8\\], 1'b0, dbg_fifo_1a_wptr\\[10:8\\]} \n\n 0110 => dbg_tp\\[7:0\\]
= {2'd0, dbg_fifo_0b_wptr\\[5:0\\]} \n\n 0111 => dbg_tp\\[7:0\\]
= {2'd0, dbg_fifo_0b_rptr\\[5:0\\]} \n\n 1000 => dbg_tp\\[7:0\\]
= {2'd0, dbg_fifo_1b_wptr\\[5:0\\]} \n\n 1001 => dbg_tp\\[7:0\\]
= {2'd0, dbg_fifo_1b_rptr\\[5:0\\]} \n\n 1010 => dbg_tp\\[7:0\\]
= {2'd0, dbg_afifo_0_rptr\\[1:0\\], 2'd0, dbg_afifo_0_wptr\\[1:0\\]} \n\n 1011 => dbg_tp\\[7:0\\]
={2'd0, dbg_afifo_1_rptr\\[1:0\\], 2'd0, dbg_afifo_1_wptr\\[1:0\\]} \n\n 1100 => dbg_tp\\[7:0\\]
= {PCM_DATA_L\\[7:1\\],VALID} \n\n 1101 => dbg_tp\\[7:0\\]
= {PCM_DATA_L\\[15:9\\],VALID} \n\n 1110 => dbg_tp\\[7:0\\]
= {PCM_DATA_R\\[7:1\\],VALID} \n\n 1111 => dbg_tp\\[7:0\\]
= {PCM_DATA_R\\[15:9\\],VALID}"]
    #[inline(always)]
    pub fn dbg_mux(&self) -> DBG_MUX_R {
        DBG_MUX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - \n 0000 => dbg_tp\\[7:0\\]
= dbg_fifo_0a_wptr\\[7:0\\]
\n 0001 => dbg_tp\\[7:0\\]
= dbg_fifo_0a_rptr\\[7:0\\]
\n 0010 => dbg_tp\\[7:0\\]
= {1'b0, dbg_fifo_0a_rptr\\[10:8\\], 1'b0, dbg_fifo_0a_wptr\\[10:8\\]} \n\n 0011 => dbg_tp\\[7:0\\]
= dbg_fifo_1a_wptr\\[7:0\\]
\n 0100 => dbg_tp\\[7:0\\]
= dbg_fifo_1a_rptr\\[7:0\\]
\n 0101 => dbg_tp\\[7:0\\]
= {1'b0, dbg_fifo_1a_rptr\\[10:8\\], 1'b0, dbg_fifo_1a_wptr\\[10:8\\]} \n\n 0110 => dbg_tp\\[7:0\\]
= {2'd0, dbg_fifo_0b_wptr\\[5:0\\]} \n\n 0111 => dbg_tp\\[7:0\\]
= {2'd0, dbg_fifo_0b_rptr\\[5:0\\]} \n\n 1000 => dbg_tp\\[7:0\\]
= {2'd0, dbg_fifo_1b_wptr\\[5:0\\]} \n\n 1001 => dbg_tp\\[7:0\\]
= {2'd0, dbg_fifo_1b_rptr\\[5:0\\]} \n\n 1010 => dbg_tp\\[7:0\\]
= {2'd0, dbg_afifo_0_rptr\\[1:0\\], 2'd0, dbg_afifo_0_wptr\\[1:0\\]} \n\n 1011 => dbg_tp\\[7:0\\]
={2'd0, dbg_afifo_1_rptr\\[1:0\\], 2'd0, dbg_afifo_1_wptr\\[1:0\\]} \n\n 1100 => dbg_tp\\[7:0\\]
= {PCM_DATA_L\\[7:1\\],VALID} \n\n 1101 => dbg_tp\\[7:0\\]
= {PCM_DATA_L\\[15:9\\],VALID} \n\n 1110 => dbg_tp\\[7:0\\]
= {PCM_DATA_R\\[7:1\\],VALID} \n\n 1111 => dbg_tp\\[7:0\\]
= {PCM_DATA_R\\[15:9\\],VALID}"]
    #[inline(always)]
    pub fn dbg_mux(&mut self) -> DBG_MUX_W {
        DBG_MUX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_mux_cfg](index.html) module"]
pub struct DBG_MUX_CFG_SPEC;
impl crate::RegisterSpec for DBG_MUX_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_mux_cfg::R](R) reader structure"]
impl crate::Readable for DBG_MUX_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_mux_cfg::W](W) writer structure"]
impl crate::Writable for DBG_MUX_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBG_MUX_CFG to value 0"]
impl crate::Resettable for DBG_MUX_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
