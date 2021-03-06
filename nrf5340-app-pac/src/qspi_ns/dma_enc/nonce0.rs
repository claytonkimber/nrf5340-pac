#[doc = "Writer for register NONCE0"]
pub type W = crate::W<u32, super::NONCE0>;
#[doc = "Register NONCE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::NONCE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `NONCE0`"]
pub struct NONCE0_W<'a> {
    w: &'a mut W,
}
impl<'a> NONCE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits 31:0 of DMA NONCE"]
    #[inline(always)]
    pub fn nonce0(&mut self) -> NONCE0_W {
        NONCE0_W { w: self }
    }
}
