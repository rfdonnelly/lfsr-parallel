require 'test_helper'

class TestLFSR < Minitest::Test
  def test_lfsr
    state = unroll_lfsr(data_size: 8, state_size: 8, polynomial: 0x07)
    state = reduce(state)
    actual = state_to_s(state)
    expected = <<~EOF.strip
      c[0] = d[0] ^ d[6] ^ d[7]
      c[1] = d[0] ^ d[1] ^ d[6]
      c[2] = d[0] ^ d[1] ^ d[2] ^ d[6]
      c[3] = d[1] ^ d[2] ^ d[3] ^ d[7]
      c[4] = d[2] ^ d[3] ^ d[4]
      c[5] = d[3] ^ d[4] ^ d[5]
      c[6] = d[4] ^ d[5] ^ d[6]
      c[7] = d[5] ^ d[6] ^ d[7]
    EOF
    assert_equal expected, actual
  end
end
