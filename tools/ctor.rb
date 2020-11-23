module Types
    IDENT = :ident
    QUALI = :quali
    CTYPE = :ctype
end

class Token
    @@registered_tokens = {}

    @sym
    @type
    @allowed_next

    attr_reader :type, :allowed_next, :sym

    def initialize sym, type, allowed_next
        @sym = sym
        @type = type
        @allowed_next = allowed_next

        @@registered_tokens[sym] = self
    end
    def Token.new_batch hash
        hash.each {|(tk, type), allowed| Token.new tk, type, allowed }
    end
    def Token.get sym
        if sym.instance_of? Token
            return Token.get sym.sym
        end
        return @@registered_tokens[sym]
    end
    def Token.registered_tokens
        return @@registered_tokens
    end
    def next(tks, i)
        tk = Token.get(tks[i])
        unless tk
            raise "Token #{tks[i]} not found"
        end
        puts tk.sym
        if @allowed_next.contains tk or @allowed_next.contains tk.type
            tk.next(tks, i+1)
        else
            raise "Did not expect token #{tk.to_sym} after #{self.type}"
        end
    end
end

Token.new_batch Hash[
    [:const, Types::QUALI] => [Types::CTYPE],
    [:*, Types::QUALI] => [Types::IDENT],
    [:[], Types::QUALI] => [],
    [:unsigned, Types::QUALI] => [Types::CTYPE]
]

END {
    ex = "unsigned int fimose".scan(%r/\w+|\S/)
    ex = ex.map {|s| s.to_sym}
    puts "Registered tokens: #{Token.registered_tokens.keys}"
    Token.get(ex[0]).next(ex, 1)
}