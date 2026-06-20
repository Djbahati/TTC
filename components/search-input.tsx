import { Search } from "lucide-react"
import { Card, CardContent } from "@/components/ui/card"
import { Input } from "@/components/ui/input"

interface SearchInputProps {
  placeholder: string
  value: string
  onChange: (value: string) => void
  className?: string
}

export function SearchInput({ placeholder, value, onChange, className }: SearchInputProps) {
  return (
    <Card className={`bg-neutral-900 border-neutral-700 ${className ?? ""}`}>
      <CardContent className="p-4">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-neutral-400" />
          <Input
            placeholder={placeholder}
            value={value}
            onChange={(e) => onChange(e.target.value)}
            className="pl-10 bg-neutral-800 border-neutral-600 text-white placeholder-neutral-400"
          />
        </div>
      </CardContent>
    </Card>
  )
}
