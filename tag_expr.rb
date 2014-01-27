class TagExpr
  def self.parse(expr)
    return TagExpr.new('t', []) if expr.empty?
    seq = []
    stack = []
    "(#{expr})".scan(/\(|\)|!|\||[^\s\(\)!\|]+/) do |token|
      case token
      when '('
        seq << token
        stack << seq.length
      when '|'
        seq << token
      when '!'
        if seq.last == '!'
          seq.pop
        else
          seq << '!'
        end
      when ')'
        exprs = []
        subexprs = []
        (seq.pop(seq.length - stack.pop) << '|').each do |item|
          if item == '|'
            raise if subexprs.empty?
            exprs << if subexprs.length == 1
                       subexprs[0]
                     else
                       TagExpr.new('&', subexprs)
                     end
            subexprs = []
          else
            subexprs << item
          end
        end
        seq[-1] = if exprs.length == 1
                    exprs[0]
                  else
                    TagExpr.new '|', exprs
                  end
      else
        seq << TagExpr.new(nil, [token])
      end
      if seq.last.is_a?(TagExpr) and seq[-2] == '!'
        seq[-1] = TagExpr.new '!', [seq.pop]
      end
    end
    raise if seq.length != 1
    return seq[0]
  end

  def initialize(operator, operands)
    @operator = operator
    @operands = operands
  end

  def verify(tags)
    case @operator
    when nil; tags.include? @operands[0]
    when '!'; not @operands[0].verify tags
    when '&'; @operands.all? { |expr| expr.verify tags }
    when '|'; @operands.any? { |expr| expr.verify tags }
    when 't'; true
    else    ; false
    end
  end
end
