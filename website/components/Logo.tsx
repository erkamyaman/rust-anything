export default function Logo({ size = 32, text = true }: { size?: number; text?: boolean }) {
  return (
    <span className="inline-flex items-center gap-2">
      <img
        src="/logo-mark.png"
        width={size}
        height={size}
        alt={text ? '' : 'Rust Anything'}
        className="shrink-0"
      />
      {text && (
        <span className="font-display text-[19px] font-semibold leading-none tracking-[-0.01em] text-fg">
          <span className="text-rust">Rust</span> Anything
        </span>
      )}
    </span>
  )
}
