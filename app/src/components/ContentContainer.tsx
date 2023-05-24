import Text from './Text'
import NavElement from '@/components/nav-element'
interface Props {
    children: React.ReactNode
}

export const ContentContainer: React.FC<Props> = ({ children }) => {
    return (
        <div className="flex-1 drawer h-52">
            <input
                id="my-drawer"
                type="checkbox"
                className="grow drawer-toggle"
            />
            <div className="items-center h-full drawer-content">{children}</div>
            {/* SideBar / Drawer */}
            <div className="drawer-side">
                <label
                    htmlFor="my-drawer"
                    className="drawer-overlay gap-6"
                ></label>

                <ul className="p-4 overflow-y-auto menu w-80 bg-base-100 gap-10 sm:flex items-center">
                    <li>
                        <Text
                            variant="heading"
                            className="font-extrabold tracking-tighter text-center text-transparent bg-clip-text bg-gradient-to-br from-indigo-500 to-fuchsia-500 mt-10"
                        >
                            Menu
                        </Text>
                    </li>
                    <li>
                        <NavElement label="Home" href="/" />
                    </li>
                </ul>
            </div>
        </div>
    )
}
