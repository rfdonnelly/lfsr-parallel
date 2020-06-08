require 'set'

INITIAL_STATE_OFFSET = 1 << 31

def polynomial_to_bits(state_size, polynomial)
  (0...state_size).map do |i|
    mask = 1 << i
    (polynomial & mask) > 0
  end
end

def state_to_s(state_bits)
  state_bits.map.with_index do |terms, i|
    terms_strings =
      terms.sort.map do |term|
        if term >= INITIAL_STATE_OFFSET
          'is[%d]' % term - INITIAL_STATE_OFFSET
        else
          'd[%d]' % term
        end
      end
    'c[%d] = %s' % [i, terms_strings.join(' ^ ')]
  end
    .join("\n")
end

class Terms
  include Enumerable

  def initialize(initial_set = [])
    @terms = Set.new(initial_set)
  end

  def add_term(term)
    if @terms.add?(term).nil?
      @terms.delete(term)
    end

    self
  end

  def add_terms(terms)
    terms.each { |term| add_term(term) }
  end

  def each(&block)
    @terms.each(&block)
  end
end

def unroll_lfsr(data_size:, state_size:, polynomial:, has_variable_initial_state: false)
  polynomial_bits = polynomial_to_bits(state_size, polynomial)

  state_bits =
    if has_variable_initial_state
      (0...state_size).map { |i| Terms.new([i + INITIAL_STATE_OFFSET]) }
    else
      (0...state_size).map { Terms.new }
    end

  (0...data_size).reverse_each do |data_bit_idx|
    state_msb = state_bits.last
    data_bit = data_bit_idx
    feedback = state_msb.add_term(data_bit)

    (1...state_size).reverse_each do |state_bit_idx|
      state_bits[state_bit_idx] = state_bits[state_bit_idx - 1]

      if polynomial_bits[state_bit_idx]
        state_bits[state_bit_idx].add_terms(feedback)
      end
    end
    state_bits[0] = feedback
  end

  state_bits
end
