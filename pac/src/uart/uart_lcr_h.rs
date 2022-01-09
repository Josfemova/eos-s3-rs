#[doc = "Register `UART_LCR_H` reader"]
pub struct R(crate::R<UART_LCR_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_LCR_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_LCR_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_LCR_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_LCR_H` writer"]
pub struct W(crate::W<UART_LCR_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_LCR_H_SPEC>;
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
impl From<crate::W<UART_LCR_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_LCR_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRK` reader - Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
pub struct BRK_R(crate::FieldReader<bool, bool>);
impl BRK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BRK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK` writer - Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
pub struct BRK_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK_W<'a> {
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
#[doc = "Parity enable. See Table 3-11 on page 3-14 for the parity truth table.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN_A {
    #[doc = "0: parity is disabled and no parity bit added to the data frame"]
    PARITY_DISABLE = 0,
    #[doc = "1: parity checking and generation is enabled."]
    PARITY_ENABLE = 1,
}
impl From<PEN_A> for bool {
    #[inline(always)]
    fn from(variant: PEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEN` reader - Parity enable. See Table 3-11 on page 3-14 for the parity truth table."]
pub struct PEN_R(crate::FieldReader<bool, PEN_A>);
impl PEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN_A {
        match self.bits {
            false => PEN_A::PARITY_DISABLE,
            true => PEN_A::PARITY_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PARITY_DISABLE`"]
    #[inline(always)]
    pub fn is_parity_disable(&self) -> bool {
        **self == PEN_A::PARITY_DISABLE
    }
    #[doc = "Checks if the value of the field is `PARITY_ENABLE`"]
    #[inline(always)]
    pub fn is_parity_enable(&self) -> bool {
        **self == PEN_A::PARITY_ENABLE
    }
}
impl core::ops::Deref for PEN_R {
    type Target = crate::FieldReader<bool, PEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN` writer - Parity enable. See Table 3-11 on page 3-14 for the parity truth table."]
pub struct PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "parity is disabled and no parity bit added to the data frame"]
    #[inline(always)]
    pub fn parity_disable(self) -> &'a mut W {
        self.variant(PEN_A::PARITY_DISABLE)
    }
    #[doc = "parity checking and generation is enabled."]
    #[inline(always)]
    pub fn parity_enable(self) -> &'a mut W {
        self.variant(PEN_A::PARITY_ENABLE)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Even parity select. Controls the type of parity the UART uses during transmission and reception. This bit has no effect when the PEN bit disables parity checking and generation. See Table 3-11 on page 3-14 for the parity truth table.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPS_A {
    #[doc = "0: Select odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits."]
    ODD_PARITY = 0,
    #[doc = "1: Select even parity. The UART generates or checks for an even number of 1s in the data and parity bits."]
    EVEN_PARITY = 1,
}
impl From<EPS_A> for bool {
    #[inline(always)]
    fn from(variant: EPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPS` reader - Even parity select. Controls the type of parity the UART uses during transmission and reception. This bit has no effect when the PEN bit disables parity checking and generation. See Table 3-11 on page 3-14 for the parity truth table."]
pub struct EPS_R(crate::FieldReader<bool, EPS_A>);
impl EPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPS_A {
        match self.bits {
            false => EPS_A::ODD_PARITY,
            true => EPS_A::EVEN_PARITY,
        }
    }
    #[doc = "Checks if the value of the field is `ODD_PARITY`"]
    #[inline(always)]
    pub fn is_odd_parity(&self) -> bool {
        **self == EPS_A::ODD_PARITY
    }
    #[doc = "Checks if the value of the field is `EVEN_PARITY`"]
    #[inline(always)]
    pub fn is_even_parity(&self) -> bool {
        **self == EPS_A::EVEN_PARITY
    }
}
impl core::ops::Deref for EPS_R {
    type Target = crate::FieldReader<bool, EPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPS` writer - Even parity select. Controls the type of parity the UART uses during transmission and reception. This bit has no effect when the PEN bit disables parity checking and generation. See Table 3-11 on page 3-14 for the parity truth table."]
pub struct EPS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits."]
    #[inline(always)]
    pub fn odd_parity(self) -> &'a mut W {
        self.variant(EPS_A::ODD_PARITY)
    }
    #[doc = "Select even parity. The UART generates or checks for an even number of 1s in the data and parity bits."]
    #[inline(always)]
    pub fn even_parity(self) -> &'a mut W {
        self.variant(EPS_A::EVEN_PARITY)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `STP2` reader - Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
pub struct STP2_R(crate::FieldReader<bool, bool>);
impl STP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STP2` writer - Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
pub struct STP2_W<'a> {
    w: &'a mut W,
}
impl<'a> STP2_W<'a> {
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
        self.w.bits =
            (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Enable FIFOs:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FEN_A {
    #[doc = "0: FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers"]
    DISABLE_FIFOS = 0,
    #[doc = "1: transmit and receive FIFO buffers are enabled (FIFO mode)."]
    ENABLE_FIFOS = 1,
}
impl From<FEN_A> for u8 {
    #[inline(always)]
    fn from(variant: FEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FEN` reader - Enable FIFOs:"]
pub struct FEN_R(crate::FieldReader<u8, FEN_A>);
impl FEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FEN_A> {
        match self.bits {
            0 => Some(FEN_A::DISABLE_FIFOS),
            1 => Some(FEN_A::ENABLE_FIFOS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_FIFOS`"]
    #[inline(always)]
    pub fn is_disable_fifos(&self) -> bool {
        **self == FEN_A::DISABLE_FIFOS
    }
    #[doc = "Checks if the value of the field is `ENABLE_FIFOS`"]
    #[inline(always)]
    pub fn is_enable_fifos(&self) -> bool {
        **self == FEN_A::ENABLE_FIFOS
    }
}
impl core::ops::Deref for FEN_R {
    type Target = crate::FieldReader<u8, FEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEN` writer - Enable FIFOs:"]
pub struct FEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers"]
    #[inline(always)]
    pub fn disable_fifos(self) -> &'a mut W {
        self.variant(FEN_A::DISABLE_FIFOS)
    }
    #[doc = "transmit and receive FIFO buffers are enabled (FIFO mode)."]
    #[inline(always)]
    pub fn enable_fifos(self) -> &'a mut W {
        self.variant(FEN_A::ENABLE_FIFOS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Word length. These bits indicate the number of data bits transmitted or received in a frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WLEN_A {
    #[doc = "0: Word lenght of data bits is will be configured as 5 bit"]
    USE_5_BIT_WORD = 0,
    #[doc = "1: Word lenght of data bits is will be configured as 6 bit"]
    USE_6_BIT_WORD = 1,
    #[doc = "2: Word lenght of data bits is will be configured as 7 bit"]
    USE_7_BIT_WORD = 2,
    #[doc = "3: Word lenght of data bits is will be configured as 8 bit"]
    USE_8_BIT_WORD = 3,
}
impl From<WLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: WLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WLEN` reader - Word length. These bits indicate the number of data bits transmitted or received in a frame"]
pub struct WLEN_R(crate::FieldReader<u8, WLEN_A>);
impl WLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WLEN_A {
        match self.bits {
            0 => WLEN_A::USE_5_BIT_WORD,
            1 => WLEN_A::USE_6_BIT_WORD,
            2 => WLEN_A::USE_7_BIT_WORD,
            3 => WLEN_A::USE_8_BIT_WORD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USE_5_BIT_WORD`"]
    #[inline(always)]
    pub fn is_use_5_bit_word(&self) -> bool {
        **self == WLEN_A::USE_5_BIT_WORD
    }
    #[doc = "Checks if the value of the field is `USE_6_BIT_WORD`"]
    #[inline(always)]
    pub fn is_use_6_bit_word(&self) -> bool {
        **self == WLEN_A::USE_6_BIT_WORD
    }
    #[doc = "Checks if the value of the field is `USE_7_BIT_WORD`"]
    #[inline(always)]
    pub fn is_use_7_bit_word(&self) -> bool {
        **self == WLEN_A::USE_7_BIT_WORD
    }
    #[doc = "Checks if the value of the field is `USE_8_BIT_WORD`"]
    #[inline(always)]
    pub fn is_use_8_bit_word(&self) -> bool {
        **self == WLEN_A::USE_8_BIT_WORD
    }
}
impl core::ops::Deref for WLEN_R {
    type Target = crate::FieldReader<u8, WLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLEN` writer - Word length. These bits indicate the number of data bits transmitted or received in a frame"]
pub struct WLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Word lenght of data bits is will be configured as 5 bit"]
    #[inline(always)]
    pub fn use_5_bit_word(self) -> &'a mut W {
        self.variant(WLEN_A::USE_5_BIT_WORD)
    }
    #[doc = "Word lenght of data bits is will be configured as 6 bit"]
    #[inline(always)]
    pub fn use_6_bit_word(self) -> &'a mut W {
        self.variant(WLEN_A::USE_6_BIT_WORD)
    }
    #[doc = "Word lenght of data bits is will be configured as 7 bit"]
    #[inline(always)]
    pub fn use_7_bit_word(self) -> &'a mut W {
        self.variant(WLEN_A::USE_7_BIT_WORD)
    }
    #[doc = "Word lenght of data bits is will be configured as 8 bit"]
    #[inline(always)]
    pub fn use_8_bit_word(self) -> &'a mut W {
        self.variant(WLEN_A::USE_8_BIT_WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Stick parity select. This bit has no effect when the PEN bit disables parity checking and generation. See Table 3-11 on page 3-14 for the parity truth table.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPS_A {
    #[doc = "0: Stick parity is disabled"]
    DISABLE_STICK_PARITY = 0,
    #[doc = "1: If EPS = 1 => parity bit is trasmitted and checked as a 1, If EPS = 0 => parity bit is trasmitted an checked as a 0"]
    ENABLE_STICK_PARITY = 1,
}
impl From<SPS_A> for bool {
    #[inline(always)]
    fn from(variant: SPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPS` reader - Stick parity select. This bit has no effect when the PEN bit disables parity checking and generation. See Table 3-11 on page 3-14 for the parity truth table."]
pub struct SPS_R(crate::FieldReader<bool, SPS_A>);
impl SPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPS_A {
        match self.bits {
            false => SPS_A::DISABLE_STICK_PARITY,
            true => SPS_A::ENABLE_STICK_PARITY,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_STICK_PARITY`"]
    #[inline(always)]
    pub fn is_disable_stick_parity(&self) -> bool {
        **self == SPS_A::DISABLE_STICK_PARITY
    }
    #[doc = "Checks if the value of the field is `ENABLE_STICK_PARITY`"]
    #[inline(always)]
    pub fn is_enable_stick_parity(&self) -> bool {
        **self == SPS_A::ENABLE_STICK_PARITY
    }
}
impl core::ops::Deref for SPS_R {
    type Target = crate::FieldReader<bool, SPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPS` writer - Stick parity select. This bit has no effect when the PEN bit disables parity checking and generation. See Table 3-11 on page 3-14 for the parity truth table."]
pub struct SPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stick parity is disabled"]
    #[inline(always)]
    pub fn disable_stick_parity(self) -> &'a mut W {
        self.variant(SPS_A::DISABLE_STICK_PARITY)
    }
    #[doc = "If EPS = 1 => parity bit is trasmitted and checked as a 1, If EPS = 0 => parity bit is trasmitted an checked as a 0"]
    #[inline(always)]
    pub fn enable_stick_parity(self) -> &'a mut W {
        self.variant(SPS_A::ENABLE_STICK_PARITY)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline(always)]
    pub fn brk(&self) -> BRK_R {
        BRK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parity enable. See Table 3-11 on page 3-14 for the parity truth table."]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Even parity select. Controls the type of parity the UART uses during transmission and reception. This bit has no effect when the PEN bit disables parity checking and generation. See Table 3-11 on page 3-14 for the parity truth table."]
    #[inline(always)]
    pub fn eps(&self) -> EPS_R {
        EPS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline(always)]
    pub fn stp2(&self) -> STP2_R {
        STP2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Enable FIFOs:"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - Word length. These bits indicate the number of data bits transmitted or received in a frame"]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Stick parity select. This bit has no effect when the PEN bit disables parity checking and generation. See Table 3-11 on page 3-14 for the parity truth table."]
    #[inline(always)]
    pub fn sps(&self) -> SPS_R {
        SPS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline(always)]
    pub fn brk(&mut self) -> BRK_W {
        BRK_W { w: self }
    }
    #[doc = "Bit 1 - Parity enable. See Table 3-11 on page 3-14 for the parity truth table."]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W {
        PEN_W { w: self }
    }
    #[doc = "Bit 2 - Even parity select. Controls the type of parity the UART uses during transmission and reception. This bit has no effect when the PEN bit disables parity checking and generation. See Table 3-11 on page 3-14 for the parity truth table."]
    #[inline(always)]
    pub fn eps(&mut self) -> EPS_W {
        EPS_W { w: self }
    }
    #[doc = "Bit 3 - Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline(always)]
    pub fn stp2(&mut self) -> STP2_W {
        STP2_W { w: self }
    }
    #[doc = "Bits 4:5 - Enable FIFOs:"]
    #[inline(always)]
    pub fn fen(&mut self) -> FEN_W {
        FEN_W { w: self }
    }
    #[doc = "Bits 5:6 - Word length. These bits indicate the number of data bits transmitted or received in a frame"]
    #[inline(always)]
    pub fn wlen(&mut self) -> WLEN_W {
        WLEN_W { w: self }
    }
    #[doc = "Bit 7 - Stick parity select. This bit has no effect when the PEN bit disables parity checking and generation. See Table 3-11 on page 3-14 for the parity truth table."]
    #[inline(always)]
    pub fn sps(&mut self) -> SPS_W {
        SPS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Line Control Register. This register accesses bit 29 to 22 of the UART Line Control Register, UARTLCR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_lcr_h](index.html) module"]
pub struct UART_LCR_H_SPEC;
impl crate::RegisterSpec for UART_LCR_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_lcr_h::R](R) reader structure"]
impl crate::Readable for UART_LCR_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_lcr_h::W](W) writer structure"]
impl crate::Writable for UART_LCR_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_LCR_H to value 0"]
impl crate::Resettable for UART_LCR_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
